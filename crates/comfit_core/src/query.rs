//! Compile query metadata and filter matches with custom predicates.

use crate::{
    Error,
    backend::{Backend, Match},
};

mod predicate;

pub use predicate::Predicate;

/// A compiled query and its metadata in the shared format.
pub struct Query<Q> {
    /// Runtime-specific compiled query.
    pub raw: Q,
    /// Capture names indexed by [`crate::backend::Capture::index`].
    pub capture_names: Vec<String>,
    /// Pattern metadata indexed by [`Match::pattern`].
    pub patterns: Vec<Pattern>,
    /// Metadata required for each capture across all query patterns.
    pub metadata: Vec<Metadata>,
}

/// Node metadata needed by a query capture.
#[derive(Clone, Copy, Default)]
pub struct Metadata {
    /// Maximum number of ancestor names to collect. Zero disables collection.
    pub ancestors: usize,
    /// Whether to collect ancestor depth without relying on ancestor names.
    pub depth: bool,
    /// Whether the immediate parent ID is needed for direct rainbow scopes.
    pub parent: bool,
    /// Whether to collect immediate children to exclude from injected content.
    pub children: bool,
}

/// Properties and custom predicates for one query pattern.
pub struct Pattern {
    /// Property settings in runtime order.
    pub properties: Vec<Property>,
    /// Custom predicates applied after runtime matching.
    pub predicates: Vec<Predicate>,
    /// Required local-binding state, or `None` for either state.
    pub local: Option<bool>,
    /// Compiled highlight priority, defaulting to 100.
    pub priority: i32,
    offsets: bool,
}

impl Pattern {
    /// Compile custom predicates and retain normalized pattern metadata.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Query`] for unsupported or malformed custom predicates.
    pub fn new(
        properties: Vec<Property>,
        predicates: Vec<(String, Vec<Argument>)>,
        local: Option<bool>,
    ) -> Result<Self, Error> {
        let predicates = predicates
            .into_iter()
            .map(|(operator, arguments)| Predicate::compile(&operator, arguments))
            .collect::<Result<Vec<_>, _>>()?;
        let priority = properties
            .iter()
            .find(|property| property.key == "priority")
            .and_then(|property| property.value.as_deref())
            .and_then(|value| value.parse().ok())
            .unwrap_or(100);
        let offsets = predicates
            .iter()
            .any(|predicate| matches!(predicate, Predicate::Offset { .. }));

        Ok(Self {
            properties,
            predicates,
            local,
            priority,
            offsets,
        })
    }

    /// Return the first property matching the key.
    pub fn property(&self, key: &str) -> Option<&Property> {
        self.properties.iter().find(|property| property.key == key)
    }

    /// Return the first matching property's value, if present.
    pub fn value(&self, key: &str) -> Option<&str> {
        self.property(key)
            .and_then(|property| property.value.as_deref())
    }
}

/// A query property in the shared format.
pub struct Property {
    /// Property name, such as `injection.language`.
    pub key: String,
    /// Optional literal property value.
    pub value: Option<String>,
    /// Optional capture index associated with the property.
    pub capture: Option<u32>,
}

/// A custom-predicate argument from a compiled query.
#[derive(Debug)]
pub enum Argument {
    /// A capture index in the query.
    Capture(u32),
    /// A literal string argument.
    String(String),
}

impl<Q> Query<Q> {
    /// Store a compiled query and determine the node metadata each capture needs.
    pub fn new(raw: Q, capture_names: Vec<String>, patterns: Vec<Pattern>) -> Self {
        let mut metadata = vec![Metadata::default(); capture_names.len()];

        for (name, metadata) in capture_names.iter().zip(&mut metadata) {
            metadata.depth = matches!(name.as_str(), "local.scope" | "rainbow.scope");
            metadata.parent = name == "rainbow.bracket";
            metadata.children = name == "injection.content"
                && patterns
                    .iter()
                    .any(|pattern| pattern.property("injection.include-children").is_none());
        }

        for pattern in &patterns {
            for predicate in &pattern.predicates {
                if let Predicate::Ancestor {
                    capture, parent, ..
                } = predicate
                {
                    let required = if *parent { 1 } else { usize::MAX };
                    metadata[*capture as usize].ancestors =
                        metadata[*capture as usize].ancestors.max(required);
                }
            }
        }

        Self {
            raw,
            capture_names,
            patterns,
            metadata,
        }
    }

    /// Run a query and filter its matches. Empty queries skip runtime execution.
    ///
    /// # Errors
    ///
    /// Returns runtime, predicate, or invalid-range errors.
    pub fn matches<B: Backend<Query = Q>>(
        &self,
        backend: &mut B,
        tree: &B::Tree,
        source: &str,
    ) -> Result<Vec<Match>, Error> {
        if self.patterns.is_empty() {
            return Ok(Vec::new());
        }
        self.filtered(backend.matches(self, tree, source)?, source)
    }

    /// Validate captures, filter custom predicates, and apply capture offsets.
    ///
    /// Reuses the input vector and preserves the order of accepted matches.
    /// Input indices must refer to this query.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRange`] for invalid source boundaries or offsets.
    ///
    /// # Panics
    ///
    /// Panics if a match references a pattern outside [`Self::patterns`].
    pub fn filtered(&self, mut matches: Vec<Match>, source: &str) -> Result<Vec<Match>, Error> {
        let mut accepted = 0;

        for index in 0..matches.len() {
            let matched = &mut matches[index];
            for capture in &matched.captures {
                if source.get(capture.node.range.clone()).is_none() {
                    return Err(Error::InvalidRange(capture.node.range.clone()));
                }
            }

            let pattern = &self.patterns[matched.pattern];

            if !pattern
                .predicates
                .iter()
                .all(|predicate| predicate.accepts(matched, source))
            {
                continue;
            }

            if pattern.offsets {
                for predicate in &pattern.predicates {
                    predicate.apply(matched, source)?;
                }

                for capture in &matched.captures {
                    if source.get(capture.node.range.clone()).is_none() {
                        return Err(Error::InvalidRange(capture.node.range.clone()));
                    }
                }
            }

            matches.swap(accepted, index);
            accepted += 1;
        }

        matches.truncate(accepted);
        Ok(matches)
    }
}
