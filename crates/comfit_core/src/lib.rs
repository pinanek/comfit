//! Syntax highlighting and themes with optional HTML and ANSI rendering.
//!
//! Implement [`backend::Backend`] to connect a parser to the highlighting engine.
//! All source ranges use UTF-8 byte offsets. Highlighted tokens borrow the source
//! and capture names from the highlighter.

pub mod backend;
pub mod document;
pub mod engine;
pub mod highlight;
mod language;
mod output;
pub mod query;
#[cfg(any(feature = "render-html", feature = "render-ansi"))]
mod render;
mod syntax;
pub mod theme;

#[cfg(feature = "render-html")]
pub use render::html;

pub use document::Document;
pub use engine::{Highlighter, Queries};
pub use highlight::{
    HighlightConfig, HighlightResult, Highlighted, Line, OwnedLine, OwnedToken, Token,
};
pub use rgb::RGB8;
pub use theme::{Color, Modifier, Style, Theme, Underline, UnderlineStyle};

#[cfg(feature = "render-html")]
pub use html::{RenderHtmlConfig, RenderHtmlStyle};

/// Errors from language registration, parsing, highlighting, or theme loading.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTML render options or named themes are invalid.
    #[error("invalid HTML options: {0}")]
    Html(String),
    /// A parsing runtime failed.
    #[error("Tree-sitter backend: {0}")]
    Backend(String),
    /// The requested root language is not registered.
    #[error("unknown or unloaded language: {0}")]
    UnknownLanguage(String),
    /// A language ID or alias is empty, repeated, or already registered.
    #[error("duplicate language ID or alias: {0}")]
    DuplicateLanguage(String),
    /// A query or predicate is invalid or unsupported.
    #[error("invalid query: {0}")]
    Query(String),
    /// A capture or offset does not describe a valid UTF-8 source slice.
    #[error("invalid UTF-8 range: {0:?}")]
    InvalidRange(std::ops::Range<usize>),
    /// Injection nesting exceeds 32 layers.
    #[error("injection nesting exceeds 32 layers")]
    InjectionDepth,
    /// A theme contains invalid syntax or style values.
    #[error("invalid theme: {0}")]
    Theme(String),
}
