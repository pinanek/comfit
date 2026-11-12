//! Bundled Tree-sitter grammars and highlighting queries.
//!
//! Enable language features to expose their modules, such as `rust::LANGUAGE`
//! and `rust::HIGHLIGHTS_QUERY`. The `all-languages` feature enables every language.

/// A bundled grammar and its embedded query sources.
#[derive(Clone, Copy)]
pub struct Language {
    /// Canonical language ID used for registration.
    pub id: &'static str,
    /// Human-readable language name.
    pub name: &'static str,
    /// Additional names accepted by the highlighter.
    pub aliases: &'static [&'static str],
    /// Tree-sitter parser entry point for this language.
    pub language: tree_sitter_language::LanguageFn,
    /// Highlight queries with inherited queries included.
    pub highlights: &'static str,
    /// Embedded-language queries with inherited queries included.
    pub injections: &'static str,
    /// Local scope, definition, and reference queries with inherited queries included.
    pub locals: &'static str,
    /// Bracket queries with inherited queries included.
    pub rainbows: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/languages.rs"));
