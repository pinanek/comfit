//! Parser adapters and query matches for the highlighting engine.

use std::ops::Range;

use crate::Error;

pub use crate::query::{Argument, Metadata, Pattern, Predicate, Property, Query};

/// A parser adapter that reports UTF-8 byte ranges and keeps query matches intact.
pub trait Backend {
    /// A grammar accepted by the parsing runtime.
    type Language;
    /// A compiled runtime query.
    type Query;
    /// A parsed tree with node IDs unique within that tree.
    type Tree;

    /// Compile a query and convert its pattern metadata to the shared format.
    ///
    /// # Errors
    ///
    /// Return an error for invalid query syntax or unsupported predicates or properties.
    fn compile(
        &mut self,
        language: &Self::Language,
        source: &str,
    ) -> Result<Query<Self::Query>, Error>;

    /// Parse the source using the given included UTF-8 byte ranges.
    ///
    /// Included ranges preserve source coordinates rather than concatenating text.
    ///
    /// # Errors
    ///
    /// Return an error when the runtime rejects the language, ranges, or parse.
    fn parse(
        &mut self,
        language: &Self::Language,
        source: &str,
        ranges: &[Range<usize>],
    ) -> Result<Self::Tree, Error>;

    /// Parse using an edited previous tree when supported by this backend.
    /// The default implementation performs a full parse and drops the old tree.
    fn parse_incremental(
        &mut self,
        language: &Self::Language,
        source: &str,
        ranges: &[Range<usize>],
        _previous: Option<Self::Tree>,
        _edit: Option<&crate::document::Edit>,
    ) -> Result<Self::Tree, Error> {
        self.parse(language, source, ranges)
    }

    /// Prepare source coordinates for reuse across parses of the same source version.
    ///
    /// Call again whenever the source changes, including same-length edits.
    /// Backends that need no preparation can use the default implementation.
    ///
    /// # Errors
    ///
    /// Returns an error if the source cannot be represented by the runtime.
    fn prepare_source(&mut self, _source: &str) -> Result<(), Error> {
        Ok(())
    }

    /// Parse a layer after calling [`Self::prepare_source`] for the same source version.
    ///
    /// The default implementation delegates to [`Self::parse_incremental`].
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime rejects the source, language, or ranges.
    fn parse_prepared(
        &mut self,
        language: &Self::Language,
        source: &str,
        ranges: &[Range<usize>],
        previous: Option<Self::Tree>,
        edit: Option<&crate::document::Edit>,
    ) -> Result<Self::Tree, Error> {
        self.parse_incremental(language, source, ranges, previous, edit)
    }

    /// Collect complete query matches in runtime order, using UTF-8 byte ranges.
    ///
    /// Pattern and capture indices must refer to the supplied query. Built-in text
    /// predicates are evaluated by the runtime. Custom predicates are evaluated by
    /// [`Query::filtered`].
    ///
    /// # Errors
    ///
    /// Return an error for runtime failures, invalid coordinates, or match limits.
    fn matches(
        &mut self,
        query: &Query<Self::Query>,
        tree: &Self::Tree,
        source: &str,
    ) -> Result<Vec<Match>, Error>;
}

/// A query match that preserves relationships between captures.
#[derive(Clone)]
pub struct Match {
    /// Index into [`Query::patterns`].
    pub pattern: usize,
    /// Captures in runtime order.
    pub captures: Vec<Capture>,
}

/// One captured node and its query capture index.
#[derive(Clone)]
pub struct Capture {
    /// Index into [`Query::capture_names`].
    pub index: u32,
    /// Node metadata owned by the highlighting engine.
    pub node: Node,
}

/// Node metadata shared by parser adapters, using UTF-8 byte offsets.
#[derive(Clone)]
pub struct Node {
    /// IDs are unique within one parsed tree, not across injection layers.
    pub id: usize,
    /// Parent node ID within the same tree, populated when requested by the query.
    pub parent: Option<usize>,
    /// Half-open UTF-8 byte range in the original source.
    pub range: Range<usize>,
    /// Immediate children in source order.
    pub children: Vec<Child>,
    /// Ancestor node kinds, nearest first, for structural query predicates.
    pub ancestors: Vec<String>,
    /// Numeric ancestor depth, populated when requested by the query.
    pub depth: usize,
}

/// An immediate child used to exclude injection subranges.
#[derive(Clone)]
pub struct Child {
    /// Half-open UTF-8 byte range in the original source.
    pub range: Range<usize>,
    /// Whether the grammar classifies this child as named.
    pub named: bool,
}
