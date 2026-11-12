//! Theme download sources and palette overrides for the import tool.

use std::collections::HashSet;

use anyhow::{Result, ensure};
use serde::Deserialize;

use crate::valid_id;

/// Theme sources selected for import.
#[derive(Debug, Deserialize)]
pub struct Registry {
    /// Sources in import order.
    pub themes: Vec<ThemeSource>,
}

/// A theme's download source and output name.
#[derive(Debug, Deserialize)]
pub struct ThemeSource {
    /// Output filename without its extension.
    pub id: String,
    /// Human-readable output theme name.
    pub name: String,
    /// Download URL for JSON or a VSIX archive.
    pub url: String,
    /// Optional bracket palette override. An empty list disables palette import.
    #[serde(default)]
    pub rainbow: Option<Vec<String>>,
    /// How the downloaded bytes are interpreted.
    #[serde(flatten)]
    pub source: Source,
}

/// Supported theme download formats.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Source {
    /// A directly downloaded JSON theme.
    Json,
    /// A VSIX archive containing a theme file.
    Extension {
        /// Path to the theme JSON within the archive.
        path: String,
    },
}

impl Registry {
    /// Validate theme IDs before downloading or writing imported themes.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid or duplicate theme IDs.
    pub fn validate(&self) -> Result<()> {
        let mut ids = HashSet::new();

        for source in &self.themes {
            ensure!(valid_id(&source.id), "invalid theme ID: {}", source.id);
            ensure!(ids.insert(&source.id), "duplicate theme ID: {}", source.id);
        }

        Ok(())
    }
}
