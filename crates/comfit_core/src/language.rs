//! Language registration, aliases, and shared compiled queries.

use std::{
    collections::{BTreeMap, HashSet},
    sync::{Arc, OnceLock},
};

use crate::{Error, backend::Backend, query::Query};

/// Queries for each highlighting pass. An empty string disables that pass.
#[derive(Clone, Copy, Default)]
pub struct Queries<'a> {
    /// Token style captures and precedence metadata.
    pub highlights: &'a str,
    /// Embedded-language captures and included-range metadata.
    pub injections: &'a str,
    /// Lexical scopes, definitions, and references.
    pub locals: &'a str,
    /// Bracket captures and nesting scopes.
    pub rainbows: &'a str,
}

pub(crate) struct Language<B: Backend> {
    raw: B::Language,
    compiled: OnceLock<Compiled<B>>,
    sources: Option<Queries<'static>>,
}

pub(crate) struct Compiled<B: Backend> {
    pub highlights: Query<B::Query>,
    pub injections: Query<B::Query>,
    pub locals: Query<B::Query>,
    pub rainbows: Query<B::Query>,
}

pub(crate) struct Registry<B: Backend> {
    languages: BTreeMap<String, Arc<Language<B>>>,
    aliases: BTreeMap<String, String>,
}

impl<B: Backend> Clone for Registry<B> {
    fn clone(&self) -> Self {
        Self {
            languages: self.languages.clone(),
            aliases: self.aliases.clone(),
        }
    }
}

impl<B: Backend> Registry<B> {
    pub fn new() -> Self {
        Self {
            languages: BTreeMap::new(),
            aliases: BTreeMap::new(),
        }
    }

    pub fn load(
        &mut self,
        backend: &mut B,
        id: &str,
        aliases: &[&str],
        language: B::Language,
        queries: Queries<'_>,
    ) -> Result<(), Error> {
        self.validate(id, aliases)?;
        let compiled = Self::compile(backend, &language, queries)?;
        self.insert(
            id,
            aliases,
            Language {
                raw: language,
                compiled: OnceLock::from(compiled),
                sources: None,
            },
        );
        Ok(())
    }

    pub fn load_lazy(
        &mut self,
        id: &str,
        aliases: &[&str],
        language: B::Language,
        queries: Queries<'static>,
    ) -> Result<(), Error> {
        self.validate(id, aliases)?;
        self.insert(
            id,
            aliases,
            Language {
                raw: language,
                compiled: OnceLock::new(),
                sources: Some(queries),
            },
        );
        Ok(())
    }

    fn validate(&self, id: &str, aliases: &[&str]) -> Result<(), Error> {
        let mut names = HashSet::new();
        for name in std::iter::once(id).chain(aliases.iter().copied()) {
            if name.is_empty()
                || !names.insert(name)
                || self.languages.contains_key(name)
                || self.aliases.contains_key(name)
            {
                return Err(Error::DuplicateLanguage(name.to_owned()));
            }
        }

        Ok(())
    }

    fn compile(
        backend: &mut B,
        language: &B::Language,
        queries: Queries<'_>,
    ) -> Result<Compiled<B>, Error> {
        Ok(Compiled {
            highlights: backend.compile(language, queries.highlights)?,
            injections: backend.compile(language, queries.injections)?,
            locals: backend.compile(language, queries.locals)?,
            rainbows: backend.compile(language, queries.rainbows)?,
        })
    }

    fn insert(&mut self, id: &str, aliases: &[&str], language: Language<B>) {
        self.languages.insert(id.to_owned(), Arc::new(language));
        for alias in aliases {
            self.aliases.insert((*alias).to_owned(), id.to_owned());
        }
    }

    pub fn get(&self, name: &str) -> Option<(&str, &Language<B>)> {
        let id = self.aliases.get(name).map_or(name, String::as_str);
        self.languages
            .get_key_value(id)
            .map(|(id, language)| (id.as_str(), language.as_ref()))
    }

    pub fn resolve(&self, name: &str) -> Result<&str, Error> {
        self.get(name)
            .map(|(id, _)| id)
            .ok_or_else(|| Error::UnknownLanguage(name.to_owned()))
    }

    pub fn language(
        &self,
        backend: &mut B,
        id: &str,
    ) -> Result<(&B::Language, &Compiled<B>), Error> {
        let language = &self.languages[id];
        if language.compiled.get().is_none() {
            let compiled = Self::compile(
                backend,
                &language.raw,
                language.sources.expect("lazy query sources"),
            )?;
            // Another document may have compiled this shared language concurrently.
            let _ = language.compiled.set(compiled);
        }
        Ok((
            &language.raw,
            language.compiled.get().expect("compiled above"),
        ))
    }
}
