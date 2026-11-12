//! Pinned grammar sources and language metadata for build tools.

use std::{
    collections::{BTreeMap, HashSet},
    path::{Component, Path},
};

use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

use crate::valid_id;

/// Query passes generated for every registered language.
pub const QUERY_KINDS: [&str; 4] = ["highlights", "injections", "locals", "rainbows"];

/// Grammar sources and language definitions keyed by stable IDs.
#[derive(Debug, Deserialize, Serialize)]
pub struct Registry {
    /// Parser sources shared by one or more languages.
    pub grammars: BTreeMap<String, Grammar>,
    /// Language definitions with grammar and query references.
    pub languages: BTreeMap<String, Language>,
}

/// A pinned grammar source and parser entry point for Rust and WASM builds.
#[derive(Debug, Deserialize, Serialize)]
pub struct Grammar {
    /// GitHub repository URL.
    pub repository: String,
    /// Pinned full commit SHA, validated by the generation tool.
    pub revision: String,
    /// Ship a compressed C parser instead of generating it during Cargo builds.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub prebuilt: bool,
    /// Repository-relative grammar directory.
    pub path: String,
    /// Generated C parser entry-point symbol.
    pub symbol: String,
}

/// A language definition that refers to a registered grammar.
#[derive(Debug, Deserialize, Serialize)]
pub struct Language {
    /// Human-readable language name.
    pub name: String,
    /// Key in [`Registry::grammars`].
    pub grammar: String,
    /// Additional names accepted by highlighters.
    pub aliases: Vec<String>,
    /// Associated file extensions.
    pub extensions: Vec<String>,
}

impl Registry {
    /// Validate IDs, pinned revisions, parser paths, aliases, and file extensions.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed entries, unknown grammar references, or
    /// duplicate aliases or extensions.
    pub fn validate(&self) -> Result<()> {
        let mut identifiers: HashSet<_> = self.languages.keys().map(String::as_str).collect();
        let mut extensions = HashSet::new();

        for (id, grammar) in &self.grammars {
            ensure!(valid_id(id), "invalid grammar ID: {id}");
            ensure!(
                grammar.revision.len() == 40
                    && grammar
                        .revision
                        .bytes()
                        .all(|byte| byte.is_ascii_hexdigit()),
                "grammar {id} must pin a full commit SHA"
            );
            ensure!(
                !grammar.path.is_empty()
                    && Path::new(&grammar.path)
                        .components()
                        .all(|part| matches!(part, Component::Normal(_) | Component::CurDir)),
                "invalid grammar path for {id}"
            );
            ensure!(
                grammar
                    .symbol
                    .strip_prefix("tree_sitter_")
                    .is_some_and(valid_id)
                    && !grammar.symbol.contains('-'),
                "invalid grammar symbol for {id}"
            );
        }

        for (id, language) in &self.languages {
            ensure!(valid_id(id), "invalid language ID: {id}");
            ensure!(
                self.grammars.contains_key(&language.grammar),
                "unknown grammar {} for {id}",
                language.grammar
            );

            for alias in &language.aliases {
                ensure!(
                    !alias.is_empty() && identifiers.insert(alias),
                    "duplicate or empty alias: {alias}"
                );
            }

            for extension in &language.extensions {
                ensure!(
                    !extension.is_empty() && extensions.insert(extension),
                    "duplicate or empty extension: {extension}"
                );
            }
        }

        Ok(())
    }
}
