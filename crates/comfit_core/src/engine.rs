//! Register languages and highlight source code, including embedded languages.

use crate::{
    Error, HighlightConfig, Highlighted, backend::Backend, document::Document, highlight,
    language::Registry,
};

pub use crate::language::Queries;

/// A highlighter with a parser backend and a language registry.
///
/// Queries are compiled during registration or on first use, then reused.
/// Results borrow the source and capture names, not the temporary syntax trees.
/// One-shot highlighting releases each layer's tree before parsing the next layer.
pub struct Highlighter<B: Backend> {
    backend: B,
    registry: Registry<B>,
}

impl<B: Backend> Highlighter<B> {
    /// Create an empty registry using the supplied backend.
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            registry: Registry::new(),
        }
    }

    /// Register a language and compile its queries.
    ///
    /// IDs and aliases share one namespace. Registration leaves the registry unchanged
    /// if validation or compilation fails. Query sources are borrowed only during loading.
    ///
    /// # Errors
    ///
    /// Returns [`Error::DuplicateLanguage`] for empty or conflicting names, or a
    /// backend/query error if compilation fails.
    pub fn load(
        &mut self,
        id: &str,
        aliases: &[&str],
        language: B::Language,
        queries: Queries<'_>,
    ) -> Result<(), Error> {
        self.registry
            .load(&mut self.backend, id, aliases, language, queries)
    }

    /// Register static queries to compile when the language is first used.
    /// Documents share uncompiled query sources and compiled queries with this registry.
    /// Query compilation errors are reported when the language is first used.
    ///
    /// # Errors
    ///
    /// Returns [`Error::DuplicateLanguage`] for empty or conflicting names.
    pub fn load_lazy(
        &mut self,
        id: &str,
        aliases: &[&str],
        language: B::Language,
        queries: Queries<'static>,
    ) -> Result<(), Error> {
        self.registry.load_lazy(id, aliases, language, queries)
    }

    /// Return whether a language ID or alias is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.registry.get(name).is_some()
    }

    /// Highlight source text using a registered language ID or alias.
    ///
    /// Injections into unregistered languages are skipped. Each syntax layer is parsed
    /// once. Results borrow the source and capture names from this registry.
    ///
    /// # Errors
    ///
    /// Returns an error for an unloaded root language, a parser or query failure, an
    /// invalid UTF-8 range, or injection nesting beyond 32 layers.
    pub fn highlight<'a>(
        &'a mut self,
        source: &'a str,
        name: &str,
        config: &HighlightConfig,
    ) -> Result<Highlighted<'a>, Error> {
        highlight::source(&mut self.backend, &self.registry, source, name, config)
    }

    /// Create an independent document using a fresh backend and shared compiled queries.
    pub fn create_document(
        &self,
        name: &str,
        source: &str,
        backend: B,
    ) -> Result<Document<B>, Error> {
        Document::new(self.registry.clone(), backend, name, source)
    }
}
