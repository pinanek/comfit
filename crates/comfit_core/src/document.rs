//! Source documents that reuse syntax trees and cache highlighting.

use crate::{
    Error, HighlightConfig, HighlightResult, backend::Backend, highlight, language::Registry,
    syntax::Syntax,
};

pub use crate::syntax::Edit;

/// A source document that shares compiled queries with its language registry.
///
/// The document keeps the languages loaded at creation. Source updates reuse the
/// root tree and compatible embedded-language trees. Highlighting queries rerun
/// only when the source or rainbow option changes. Dropping the document releases
/// its trees.
pub struct Document<B: Backend> {
    backend: B,
    registry: Registry<B>,
    language: String,
    source: String,
    syntax: Option<Syntax<B>>,
    cached: Option<(bool, HighlightResult)>,
}

impl<B: Backend> Document<B> {
    pub(crate) fn new(
        registry: Registry<B>,
        mut backend: B,
        name: &str,
        source: &str,
    ) -> Result<Self, Error> {
        let language = registry.resolve(name)?.to_owned();
        let syntax = Syntax::parse(&mut backend, &registry, &language, source, None, None)?;
        Ok(Self {
            backend,
            registry,
            language,
            source: source.to_owned(),
            syntax: Some(syntax),
            cached: None,
        })
    }

    /// Return the current source.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Replace the source and reuse compatible trees with a UTF-8-safe edit.
    /// If the source is unchanged, do nothing. A failed parse leaves the source unchanged.
    /// The source buffer retains its capacity for subsequent updates.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or query compilation fails, or injection nesting is too deep.
    pub fn update(&mut self, source: &str) -> Result<(), Error> {
        if self.source == source {
            return Ok(());
        }
        let edit = Edit::between(&self.source, source);
        let syntax = Syntax::parse(
            &mut self.backend,
            &self.registry,
            &self.language,
            source,
            self.syntax.take(),
            Some(&edit),
        )?;
        self.source.clear();
        self.source.push_str(source);
        self.syntax = Some(syntax);
        self.cached = None;
        Ok(())
    }

    /// Return an owned snapshot. Changes to line emphasis reuse cached highlighting.
    /// Snapshots remain valid after the document is edited or dropped.
    pub fn highlight(&mut self, config: &HighlightConfig) -> Result<HighlightResult, Error> {
        Ok(self.highlight_ref(config)?.clone())
    }

    /// Borrow cached highlighting without cloning tokens.
    /// The source cannot be updated while the output is borrowed.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or highlighting fails.
    pub fn highlight_ref(&mut self, config: &HighlightConfig) -> Result<&HighlightResult, Error> {
        if self
            .cached
            .as_ref()
            .is_none_or(|(rainbow, _)| *rainbow != config.rainbow)
        {
            if self.syntax.is_none() {
                self.syntax = Some(Syntax::parse(
                    &mut self.backend,
                    &self.registry,
                    &self.language,
                    &self.source,
                    None,
                    None,
                )?);
            }
            let highlighted = highlight::run(
                &mut self.backend,
                &self.registry,
                &self.source,
                self.syntax.as_ref().expect("parsed above"),
                config,
            )?;
            self.cached = Some((config.rainbow, highlighted.to_owned()));
        }
        let result = &mut self.cached.as_mut().expect("highlighted above").1;
        let mut selected = config.highlighted_lines.clone();
        selected.sort_unstable();
        let mut selected = selected.into_iter().peekable();

        for line in &mut result.lines {
            while selected.peek().is_some_and(|number| *number < line.number) {
                selected.next();
            }
            line.highlighted = selected.peek() == Some(&line.number);
        }
        Ok(result)
    }
}
