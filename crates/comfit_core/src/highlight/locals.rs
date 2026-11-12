//! Match references to local definitions and share their highlighting styles.

use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use crate::{backend::Match, query::Query};

use super::compose::Span;

/// Definition nodes and resolved references within one parsed language layer.
pub(crate) struct Bindings {
    definitions: HashSet<usize>,
    references: HashMap<usize, usize>,
}

struct Scope {
    range: Range<usize>,
    inherits: bool,
    depth: usize,
    parent: Option<usize>,
}

struct Definition<'a> {
    node: usize,
    range: Range<usize>,
    value_end: usize,
    name: &'a str,
    kind: &'a str,
    scope: usize,
}

impl Bindings {
    /// Match references to earlier definitions, following lexical scope inheritance rules.
    pub fn resolve<Q>(query: &Query<Q>, matches: &[Match], source: &str) -> Self {
        if matches.is_empty() {
            return Self {
                definitions: HashSet::new(),
                references: HashMap::new(),
            };
        }

        let mut scopes = vec![Scope {
            range: 0..source.len(),
            inherits: false,
            depth: 0,
            parent: None,
        }];
        let mut scope_ids = HashSet::new();

        for matched in matches {
            for capture in &matched.captures {
                if query.capture_names[capture.index as usize] == "local.scope"
                    && scope_ids.insert(capture.node.id)
                {
                    scopes.push(Scope {
                        range: capture.node.range.clone(),
                        inherits: query.patterns[matched.pattern].value("local.scope-inherits")
                            != Some("false"),
                        depth: capture.node.depth,
                        parent: None,
                    });
                }
            }
        }

        scopes.sort_by_key(|scope| {
            (
                scope.range.start,
                std::cmp::Reverse(scope.range.end),
                scope.depth,
            )
        });

        let mut active = vec![0];

        for index in 1..scopes.len() {
            while active.last().is_some_and(|&parent: &usize| {
                !contains(&scopes[parent].range, &scopes[index].range)
            }) {
                active.pop();
            }
            scopes[index].parent = active.last().copied();
            active.push(index);
        }

        let scope_for = |range: &Range<usize>| {
            let mut index = scopes
                .partition_point(|scope| scope.range.start <= range.start)
                .saturating_sub(1);

            while !contains(&scopes[index].range, range) {
                index = scopes[index].parent.unwrap_or(0);
            }
            index
        };

        let mut definitions = Vec::new();

        for matched in matches {
            let pattern = &query.patterns[matched.pattern];
            let value_end = matched
                .captures
                .iter()
                .find(|capture| {
                    query.capture_names[capture.index as usize] == "local.definition-value"
                })
                .map_or(0, |capture| capture.node.range.end);

            for capture in &matched.captures {
                let name = &query.capture_names[capture.index as usize];

                if name != "local.definition" && !name.starts_with("local.definition.") {
                    continue;
                }

                let kind = name.strip_prefix("local.definition.").unwrap_or("");
                let mut scope = scope_for(&capture.node.range);

                if pattern
                    .properties
                    .iter()
                    .find(|property| {
                        property
                            .key
                            .strip_prefix("definition.")
                            .and_then(|key| key.strip_suffix(".scope"))
                            == Some(kind)
                    })
                    .and_then(|property| property.value.as_deref())
                    == Some("parent")
                {
                    scope = scopes[scope].parent.unwrap_or(0);
                }

                definitions.push(Definition {
                    node: capture.node.id,
                    range: capture.node.range.clone(),
                    value_end,
                    name: &source[capture.node.range.clone()],
                    kind,
                    scope,
                });
            }
        }

        definitions.sort_by_key(|definition| definition.range.start);

        let mut lookup: Vec<HashMap<&str, Vec<&Definition<'_>>>> =
            (0..scopes.len()).map(|_| HashMap::new()).collect();

        for definition in &definitions {
            lookup[definition.scope]
                .entry(definition.name)
                .or_default()
                .push(definition);
        }

        let mut result = Self {
            definitions: definitions
                .iter()
                .map(|definition| definition.node)
                .collect(),
            references: HashMap::new(),
        };

        for matched in matches {
            for capture in &matched.captures {
                if query.capture_names[capture.index as usize] != "local.reference" {
                    continue;
                }

                let name = &source[capture.node.range.clone()];
                let kind = query.patterns[matched.pattern].value("reference.kind");
                let mut current = Some(scope_for(&capture.node.range));

                while let Some(index) = current {
                    let candidates = lookup[index].get(name).map_or(&[][..], Vec::as_slice);
                    let preceding = candidates.partition_point(|definition| {
                        definition.range.start <= capture.node.range.start
                    });

                    if let Some(definition) =
                        candidates[..preceding].iter().rev().find(|definition| {
                            kind.is_none_or(|kind| kind == definition.kind)
                                && definition.value_end <= capture.node.range.start
                        })
                    {
                        result.references.insert(capture.node.id, definition.node);
                        break;
                    }

                    current = if scopes[index].inherits {
                        scopes[index].parent
                    } else {
                        None
                    };
                }
            }
        }

        result
    }

    /// Return whether a node is a definition or a reference with a resolved definition.
    pub fn is_local(&self, node: usize) -> bool {
        self.definitions.contains(&node) || self.references.contains_key(&node)
    }

    /// Apply the winning definition style to references, falling back to `variable`.
    pub fn apply<'a>(&self, spans: &mut [Span<'a>]) {
        if self.references.is_empty() {
            return;
        }

        let mut styles = HashMap::new();

        for span in spans
            .iter()
            .filter(|span| self.definitions.contains(&span.node))
        {
            let entry = styles
                .entry(span.node)
                .or_insert((span.rank, span.highlight));

            if span.rank >= entry.0 {
                *entry = (span.rank, span.highlight);
            }
        }

        for span in spans {
            if let Some(definition) = self.references.get(&span.node) {
                span.highlight = styles
                    .get(definition)
                    .and_then(|(_, highlight)| *highlight)
                    .or(Some("variable"));
            }
        }
    }
}

fn contains(outer: &Range<usize>, inner: &Range<usize>) -> bool {
    outer.start <= inner.start && inner.end <= outer.end
}
