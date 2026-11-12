//! Bundled themes selected through Cargo features.
//!
//! Each enabled theme has a module with a `theme()` function. The first call
//! initializes the theme. All calls return a shared, immutable reference that
//! can be used across threads. TOML files are parsed at build time.
//!
//! Enable `convert` to convert VS Code JSON/JSON5 themes at runtime. This feature
//! does not require any bundled themes.
//!
//! # Examples
//!
//! Enable the `catppuccin-mocha` feature to use this theme:
//!
//! ```
//! # #[cfg(feature = "catppuccin-mocha")]
//! # {
//! let theme = comfit_themes::catppuccin_mocha::theme();
//! let function = theme.style("function.call");
//! assert!(function.fg.is_some());
//! # }
//! ```

use comfit_core::Theme;

#[cfg(feature = "convert")]
pub mod convert;

/// An enabled theme's metadata and initialization function.
#[derive(Clone, Copy)]
pub struct ThemeDefinition {
    /// Theme ID, matching its feature and TOML filename stem.
    pub id: &'static str,
    /// Human-readable theme name, available without initializing the theme.
    pub name: &'static str,
    /// Initialize the theme once and return the shared value.
    pub theme: fn() -> &'static Theme,
}

include!(concat!(env!("OUT_DIR"), "/themes.rs"));
