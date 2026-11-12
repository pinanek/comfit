//! Adapt web-tree-sitter queries and node metadata to the shared engine.

use std::{collections::HashMap, ops::Range, rc::Rc};

use comfit_core::{
    Error,
    backend::{Argument, Backend, Capture, Child, Match, Metadata, Node, Pattern, Property, Query},
};
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};

use crate::{binding, source::Coordinates};

/// A web-tree-sitter parser released when the backend is dropped.
pub struct WebBackend {
    parser: binding::Parser,
    coordinates: Option<Rc<Coordinates>>,
}

/// An owned runtime query released on drop.
pub struct WebQuery(binding::Query, HashMap<String, u32>);

/// A runtime tree and its source-coordinate conversion tables, released on drop.
pub struct WebTree {
    raw: binding::Tree,
    coordinates: Rc<Coordinates>,
}

impl WebBackend {
    /// Create a parser after web-tree-sitter initialization.
    ///
    /// # Errors
    ///
    /// Returns a backend error if the JavaScript parser constructor throws.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            parser: binding::Parser::new().map_err(backend_error)?,
            coordinates: None,
        })
    }
}

impl Backend for WebBackend {
    type Language = binding::Language;
    type Query = WebQuery;
    type Tree = WebTree;

    fn compile(
        &mut self,
        language: &Self::Language,
        source: &str,
    ) -> Result<Query<Self::Query>, Error> {
        let mut raw = WebQuery(
            binding::Query::new(language, source).map_err(backend_error)?,
            HashMap::new(),
        );
        let capture_names: Vec<_> = raw
            .0
            .capture_names()
            .iter()
            .map(|name| {
                name.as_string()
                    .ok_or_else(|| Error::Query("invalid capture name".into()))
            })
            .collect::<Result<_, _>>()?;
        raw.1 = capture_names
            .iter()
            .enumerate()
            .map(|(index, name)| (name.clone(), index as u32))
            .collect();
        let settings = raw.0.settings();
        let refuted = raw.0.refuted_properties();
        let asserted = raw.0.asserted_properties();
        let mut patterns = Vec::new();

        for index in 0..raw.0.pattern_count() {
            let mut properties = Vec::new();
            let value = settings.get(index);

            if !value.is_null() && !value.is_undefined() {
                for entry in Object::entries(value.unchecked_ref()).iter() {
                    let entry = Array::from(&entry);
                    properties.push(Property {
                        key: entry.get(0).as_string().unwrap(),
                        value: entry.get(1).as_string(),
                        capture: None,
                    });
                }
            }

            let mut predicates = Vec::new();

            for value in raw.0.predicates_for_pattern(index) {
                let predicate: binding::PredicateResult = value.unchecked_into();
                let mut arguments = Vec::new();

                for value in predicate.operands() {
                    let operand: binding::PredicateOperand = value.unchecked_into();
                    arguments.push(if operand.type_() == "capture" {
                        let name = String::from(operand.name());
                        Argument::Capture(
                            raw.1
                                .get(&name)
                                .copied()
                                .ok_or_else(|| Error::Query(format!("unknown capture: {name}")))?,
                        )
                    } else {
                        Argument::String(String::from(operand.value()))
                    });
                }
                predicates.push((String::from(predicate.operator()), arguments));
            }

            let mut local = None;

            for (value, positive) in [(refuted.get(index), false), (asserted.get(index), true)] {
                if value.is_null() || value.is_undefined() {
                    continue;
                }

                for key in Object::keys(value.unchecked_ref::<Object>()).iter() {
                    if key.as_string().as_deref() != Some("local") {
                        return Err(Error::Query("unsupported property predicate".into()));
                    }
                    local = Some(positive);
                }
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
        self.coordinates = Some(Rc::new(Coordinates::new(source)?));
        Ok(())
    }

    fn parse_prepared(
        &mut self,
        language: &Self::Language,
        source: &str,
        ranges: &[Range<usize>],
        previous: Option<Self::Tree>,
        edit: Option<&comfit_core::document::Edit>,
    ) -> Result<Self::Tree, Error> {
        let coordinates = Rc::clone(
            self.coordinates
                .as_ref()
                .expect("source prepared before parsing"),
        );
        if let (Some(tree), Some(edit)) = (&previous, edit) {
            let old = tree
                .coordinates
                .web_range(&(edit.start_byte..edit.old_end_byte));
            let new = coordinates.web_range(&(edit.start_byte..edit.new_end_byte));
            let input = Object::new();
            for (key, range, field) in [
                ("startIndex", &old, "startIndex"),
                ("oldEndIndex", &old, "endIndex"),
                ("newEndIndex", &new, "endIndex"),
                ("startPosition", &old, "startPosition"),
                ("oldEndPosition", &old, "endPosition"),
                ("newEndPosition", &new, "endPosition"),
            ] {
                let value = Reflect::get(range, &field.into()).map_err(backend_error)?;
                Reflect::set(&input, &key.into(), &value).map_err(backend_error)?;
            }
            tree.raw.edit(&input).map_err(backend_error)?;
        }
        let included = Array::new();

        for range in ranges.iter().filter(|range| !range.is_empty()) {
            included.push(&coordinates.web_range(range));
        }

        let options = binding::ParseOptions::new(Some(&included));
        self.parser
            .set_language(Some(language))
            .map_err(backend_error)?;
        let raw = self
            .parser
            .parse_with_string(
                source,
                previous.as_ref().map(|tree| &tree.raw),
                Some(&options),
            )
            .map_err(backend_error)?
            .ok_or_else(|| Error::Backend("parser returned no tree".into()))?;

        Ok(WebTree { raw, coordinates })
    }

    fn matches(
        &mut self,
        query: &Query<Self::Query>,
        tree: &Self::Tree,
        source: &str,
    ) -> Result<Vec<Match>, Error> {
        let mut result = Vec::new();

        for value in query
            .raw
            .0
            .matches(&tree.raw.root_node(), None)
            .map_err(backend_error)?
            .iter()
        {
            let matched: binding::QueryMatch = value.unchecked_into();
            let mut captures = Vec::new();

            for value in matched.captures() {
                let capture: binding::QueryCapture = value.unchecked_into();
                let name = String::from(capture.name());
                let index = query
                    .raw
                    .1
                    .get(&name)
                    .copied()
                    .ok_or_else(|| Error::Query(format!("unknown capture: {name}")))?;
                captures.push(Capture {
                    index,
                    node: node(
                        &capture.node(),
                        &tree.coordinates,
                        source,
                        query.metadata[index as usize],
                    )?,
                });
            }

            result.push(Match {
                pattern: matched.pattern() as usize,
                captures,
            });
        }

        if query.raw.0.did_exceed_match_limit() {
            return Err(Error::Backend("query match limit exceeded".into()));
        }

        Ok(result)
    }
}

fn node(
    raw: &binding::SyntaxNode,
    coordinates: &Coordinates,
    source: &str,
    metadata: Metadata,
) -> Result<Node, Error> {
    let mut children = Vec::new();

    if metadata.children {
        for value in raw.children() {
            let child: binding::SyntaxNode = value.unchecked_into();
            children.push(Child {
                range: coordinates.range(&child, source)?,
                named: child.is_named(),
            });
        }
    }

    let mut ancestors = Vec::new();
    let mut depth = 0;
    let mut parent = if metadata.parent || metadata.depth || metadata.ancestors > 0 {
        raw.parent()
    } else {
        None
    };
    let parent_id = if metadata.parent {
        parent.as_ref().map(|parent| parent.id() as usize)
    } else {
        None
    };

    while (metadata.depth || depth < metadata.ancestors)
        && let Some(node) = parent
    {
        if depth < metadata.ancestors {
            ancestors.push(String::from(node.type_()));
        }
        depth += 1;
        parent = node.parent();
    }

    Ok(Node {
        id: raw.id() as usize,
        parent: parent_id,
        range: coordinates.range(raw, source)?,
        children,
        ancestors,
        depth,
    })
}

/// Preserve a thrown string or JavaScript error message as a shared backend error.
pub fn backend_error(value: JsValue) -> Error {
    let message = value
        .as_string()
        .or_else(|| {
            Reflect::get(&value, &"message".into())
                .ok()
                .and_then(|message| message.as_string())
        })
        .unwrap_or_else(|| format!("{value:?}"));

    Error::Backend(message)
}

impl Drop for WebBackend {
    fn drop(&mut self) {
        self.parser.delete();
    }
}

impl Drop for WebQuery {
    fn drop(&mut self) {
        self.0.delete();
    }
}

impl Drop for WebTree {
    fn drop(&mut self) {
        self.raw.delete();
    }
}
