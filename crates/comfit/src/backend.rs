//! Tree-sitter adapter with reusable parser, query cursor, and line offsets.

use std::ops::Range;

use comfit_core::{
    Error,
    backend::{Argument, Backend, Capture, Child, Match, Metadata, Node, Pattern, Property, Query},
};
use streaming_iterator::StreamingIterator;

/// Reusable Tree-sitter parser and query cursor.
///
/// Matches contain owned node metadata. Trees do not borrow the parser.
/// Line-offset storage is retained between parse calls.
#[derive(Default)]
pub struct NativeBackend {
    parser: tree_sitter::Parser,
    cursor: tree_sitter::QueryCursor,
    lines: Vec<usize>,
}

impl Backend for NativeBackend {
    type Language = tree_sitter::Language;
    type Query = tree_sitter::Query;
    type Tree = tree_sitter::Tree;

    fn compile(
        &mut self,
        language: &Self::Language,
        source: &str,
    ) -> Result<Query<Self::Query>, Error> {
        let raw = tree_sitter::Query::new(language, source)
            .map_err(|error| Error::Query(error.to_string()))?;
        let capture_names = raw
            .capture_names()
            .iter()
            .map(|name| (*name).to_owned())
            .collect();
        let mut patterns = Vec::new();

        for index in 0..raw.pattern_count() {
            let properties = raw
                .property_settings(index)
                .iter()
                .map(|property| Property {
                    key: property.key.to_string(),
                    value: property.value.as_ref().map(ToString::to_string),
                    capture: property.capture_id.map(|index| index as u32),
                })
                .collect();
            let predicates = raw
                .general_predicates(index)
                .iter()
                .map(|predicate| {
                    (
                        predicate.operator.to_string(),
                        predicate
                            .args
                            .iter()
                            .map(|argument| match argument {
                                tree_sitter::QueryPredicateArg::Capture(index) => {
                                    Argument::Capture(*index)
                                }
                                tree_sitter::QueryPredicateArg::String(value) => {
                                    Argument::String(value.to_string())
                                }
                            })
                            .collect(),
                    )
                })
                .collect();
            let mut local = None;

            for (property, positive) in raw.property_predicates(index) {
                if property.key.as_ref() != "local" {
                    return Err(Error::Query(format!(
                        "unsupported property predicate: {}",
                        property.key
                    )));
                }
                local = Some(*positive);
            }

            patterns.push(Pattern::new(properties, predicates, local)?);
        }

        Ok(Query::new(raw, capture_names, patterns))
    }

    fn parse(
        &mut self,
        language: &Self::Language,
        source: &str,
        ranges: &[Range<usize>],
    ) -> Result<Self::Tree, Error> {
        self.parse_incremental(language, source, ranges, None, None)
    }

    fn parse_incremental(
        &mut self,
        language: &Self::Language,
        source: &str,
        ranges: &[Range<usize>],
        previous: Option<Self::Tree>,
        edit: Option<&comfit_core::document::Edit>,
    ) -> Result<Self::Tree, Error> {
        self.prepare_source(source)?;
        self.parse_prepared(language, source, ranges, previous, edit)
    }

    fn prepare_source(&mut self, source: &str) -> Result<(), Error> {
        self.lines.clear();
        self.lines.push(0);
        self.lines
            .extend(source.match_indices('\n').map(|(byte, _)| byte + 1));
        Ok(())
    }

    fn parse_prepared(
        &mut self,
        language: &Self::Language,
        source: &str,
        ranges: &[Range<usize>],
        mut previous: Option<Self::Tree>,
        edit: Option<&comfit_core::document::Edit>,
    ) -> Result<Self::Tree, Error> {
        if let (Some(tree), Some(edit)) = (&mut previous, edit) {
            let point = |(row, column)| tree_sitter::Point { row, column };
            tree.edit(&tree_sitter::InputEdit {
                start_byte: edit.start_byte,
                old_end_byte: edit.old_end_byte,
                new_end_byte: edit.new_end_byte,
                start_position: point(edit.start_position),
                old_end_position: point(edit.old_end_position),
                new_end_position: point(edit.new_end_position),
            });
        }
        self.parser
            .set_language(language)
            .map_err(|error| Error::Backend(error.to_string()))?;
        let ranges = ranges
            .iter()
            .filter(|range| !range.is_empty())
            .map(|range| tree_sitter::Range {
                start_byte: range.start,
                end_byte: range.end,
                start_point: point(&self.lines, range.start),
                end_point: point(&self.lines, range.end),
            })
            .collect::<Vec<_>>();
        self.parser
            .set_included_ranges(&ranges)
            .map_err(|error| Error::Backend(error.to_string()))?;
        self.parser
            .parse(source, previous.as_ref())
            .ok_or_else(|| Error::Backend("parser returned no tree".into()))
    }

    fn matches(
        &mut self,
        query: &Query<Self::Query>,
        tree: &Self::Tree,
        source: &str,
    ) -> Result<Vec<Match>, Error> {
        let mut result = Vec::new();
        {
            let mut matches = self
                .cursor
                .matches(&query.raw, tree.root_node(), source.as_bytes());

            while let Some(matched) = matches.next() {
                result.push(Match {
                    pattern: matched.pattern_index,
                    captures: matched
                        .captures()
                        .iter()
                        .map(|capture| Capture {
                            index: capture.index,
                            node: node(capture.node, query.metadata[capture.index as usize]),
                        })
                        .collect(),
                });
            }
        }

        if self.cursor.did_exceed_match_limit() {
            return Err(Error::Backend("query match limit exceeded".into()));
        }

        Ok(result)
    }
}

fn point(lines: &[usize], byte: usize) -> tree_sitter::Point {
    let row = lines.partition_point(|start| *start <= byte) - 1;
    tree_sitter::Point::new(row, byte - lines[row])
}

fn node(raw: tree_sitter::Node<'_>, metadata: Metadata) -> Node {
    let children = if metadata.children {
        let mut cursor = raw.walk();
        raw.children(&mut cursor)
            .map(|child| Child {
                range: child.byte_range(),
                named: child.is_named(),
            })
            .collect()
    } else {
        Vec::new()
    };
    let mut ancestors = Vec::new();
    let mut depth = 0;
    let mut parent = if metadata.parent || metadata.depth || metadata.ancestors > 0 {
        raw.parent()
    } else {
        None
    };
    let parent_id = if metadata.parent {
        parent.as_ref().map(|parent| parent.id())
    } else {
        None
    };

    while (metadata.depth || depth < metadata.ancestors)
        && let Some(node) = parent
    {
        if depth < metadata.ancestors {
            ancestors.push(node.kind().to_owned());
        }
        depth += 1;
        parent = node.parent();
    }

    Node {
        id: raw.id(),
        parent: parent_id,
        range: raw.byte_range(),
        children,
        ancestors,
        depth,
    }
}
