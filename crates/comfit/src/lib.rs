//! Tree-sitter syntax highlighting for Rust applications.
//!
//! Enable language features to load bundled grammars, or register grammars with
//! [`Highlighter::load`]. Reuse a highlighter to avoid recompiling queries.
//!
//! # Examples
//!
//! With the `rust` feature enabled:
//!
//! ```
//! # #[cfg(feature = "rust")]
//! # fn main() -> Result<(), comfit::Error> {
//! let mut highlighter = comfit::Highlighter::new()?;
//! let highlighted = highlighter.highlight(
//!     "fn main() {}",
//!     "rs",
//!     &comfit::HighlightConfig::default(),
//! )?;
//! assert_eq!(highlighted.lines[0].number, 1);
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "rust"))]
//! # fn main() {}
//! ```

pub mod backend;

pub use comfit_core::{
    Color, Error, HighlightConfig, HighlightResult, Highlighted, Line, Modifier, OwnedLine,
    OwnedToken, Queries, RGB8, Style, Theme, Token, Underline, UnderlineStyle,
};
pub use comfit_grammars as grammars;

#[cfg(feature = "render-html")]
pub use comfit_core::{RenderHtmlConfig, RenderHtmlStyle};

/// A source document that incrementally updates root and embedded-language trees.
pub type Document = comfit_core::Document<backend::NativeBackend>;

/// A highlighter that reuses its Tree-sitter parser and compiled queries.
pub struct Highlighter {
    inner: comfit_core::Highlighter<backend::NativeBackend>,
}

impl Highlighter {
    /// Register bundled languages and compile their queries on first use.
    ///
    /// # Errors
    ///
    /// Returns an error if a bundled language cannot be registered. Query compilation
    /// errors are reported when highlighting or creating a document.
    pub fn new() -> Result<Self, Error> {
        let mut highlighter = Self {
            inner: comfit_core::Highlighter::new(backend::NativeBackend::default()),
        };

        for definition in grammars::LANGUAGES {
            highlighter.inner.load_lazy(
                definition.id,
                definition.aliases,
                definition.language.into(),
                Queries {
                    highlights: definition.highlights,
                    injections: definition.injections,
                    locals: definition.locals,
                    rainbows: definition.rainbows,
                },
            )?;
        }

        Ok(highlighter)
    }

    /// Register an external grammar and compile its queries.
    ///
    /// # Errors
    ///
    /// Returns an error for empty or conflicting IDs or aliases, or invalid queries.
    /// Failed registration does not change the language registry.
    pub fn load(
        &mut self,
        id: &str,
        aliases: &[&str],
        language: tree_sitter::Language,
        queries: Queries<'_>,
    ) -> Result<(), Error> {
        self.inner.load(id, aliases, language, queries)
    }

    /// Create a document that shares the loaded grammars and compiled queries.
    /// Documents and owned results remain valid after the highlighter is dropped.
    pub fn create_document(&self, language: &str, source: &str) -> Result<Document, Error> {
        self.inner
            .create_document(language, source, backend::NativeBackend::default())
    }

    /// Return whether a language ID or alias is loaded.
    pub fn contains(&self, name: &str) -> bool {
        self.inner.contains(name)
    }

    /// Highlight source using a loaded language ID or alias.
    ///
    /// Results borrow the source and this highlighter. Unloaded injected languages
    /// are skipped.
    ///
    /// # Errors
    ///
    /// Returns an error for an unloaded root language, parser or query failures, invalid
    /// UTF-8 ranges, or injection nesting beyond 32 layers.
    pub fn highlight<'a>(
        &'a mut self,
        source: &'a str,
        language: &str,
        config: &HighlightConfig,
    ) -> Result<Highlighted<'a>, Error> {
        self.inner.highlight(source, language, config)
    }
}
