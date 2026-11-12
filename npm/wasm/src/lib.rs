//! Browser Tree-sitter adapter and WASM bindings for the highlighting engine.
//!
//! The TypeScript entry point loads WASM inputs and manages instance disposal.
//! Initialize the runtime before creating a highlighter.

use std::{cell::RefCell, collections::BTreeMap};

use comfit_core::{Error, HighlightConfig, Queries, RenderHtmlConfig, RenderHtmlStyle};
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize, Serializer};
use wasm_bindgen::prelude::*;

mod backend;
mod binding;
mod source;
mod theme;

use theme::ThemeDefinition;

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum HtmlStyle {
    #[default]
    Class,
    Inline,
    Variables,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LanguageDefinition {
    id: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    highlights: String,
    #[serde(default)]
    injections: String,
    #[serde(default)]
    locals: String,
    #[serde(default)]
    rainbows: String,
}

#[derive(Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
struct Options {
    highlighted_lines: Option<Vec<usize>>,
    rainbow: bool,
    pre_tag: String,
    line_tag: String,
    pre_class: Option<String>,
    class_prefix: String,
    rainbow_colors: usize,
    style: HtmlStyle,
    theme: Option<ThemeDefinition>,
    themes: Option<BTreeMap<String, ThemeDefinition>>,
    default_theme: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            highlighted_lines: None,
            rainbow: HighlightConfig::default().rainbow,
            pre_tag: "pre".into(),
            line_tag: "span".into(),
            pre_class: None,
            class_prefix: String::new(),
            rainbow_colors: 0,
            style: HtmlStyle::Class,
            theme: None,
            themes: None,
            default_theme: None,
        }
    }
}

impl Options {
    fn highlight(&self) -> HighlightConfig {
        HighlightConfig {
            highlighted_lines: self.highlighted_lines.clone().unwrap_or_default(),
            rainbow: self.rainbow,
        }
    }

    fn render(&self) -> Result<RenderHtmlConfig, Error> {
        match self.style {
            HtmlStyle::Class | HtmlStyle::Variables => {
                if self.theme.is_some() || self.themes.is_some() || self.default_theme.is_some() {
                    return Err(Error::Html(
                        "themes are only supported with inline style".into(),
                    ));
                }
            }
            HtmlStyle::Inline => {
                if self.theme.is_some() {
                    if self.themes.is_some() || self.default_theme.is_some() {
                        return Err(Error::Html(
                            "theme cannot be combined with themes or defaultTheme".into(),
                        ));
                    }
                } else {
                    let themes = self
                        .themes
                        .as_ref()
                        .filter(|themes| !themes.is_empty())
                        .ok_or_else(|| {
                            Error::Html("inline style requires theme or nonempty themes".into())
                        })?;
                    if themes.len() > 1 && self.default_theme.is_none() {
                        return Err(Error::Html("multiple themes require defaultTheme".into()));
                    }
                }
            }
        }

        Ok(RenderHtmlConfig {
            pre_tag: self.pre_tag.clone(),
            line_tag: self.line_tag.clone(),
            pre_class: self.pre_class.clone(),
            class_prefix: self.class_prefix.clone(),
            rainbow_colors: self.rainbow_colors,
            style: match self.style {
                HtmlStyle::Class => RenderHtmlStyle::Class,
                HtmlStyle::Inline => RenderHtmlStyle::Inline,
                HtmlStyle::Variables => RenderHtmlStyle::Variables,
            },
        })
    }
}

#[derive(Serialize)]
struct JsonToken<'a> {
    highlight: Option<&'a str>,
    rainbow: Option<usize>,
    text: &'a str,
}

#[derive(Serialize)]
struct JsonLine<'a, 'source> {
    highlighted: bool,
    number: usize,
    #[serde(serialize_with = "serialize_tokens")]
    tokens: &'a [comfit_core::Token<'source>],
}

fn serialize_tokens<S: Serializer>(
    tokens: &[comfit_core::Token<'_>],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(tokens.iter().map(|token| JsonToken {
        highlight: token.highlight,
        rainbow: token.rainbow,
        text: token.text,
    }))
}

#[derive(Serialize)]
struct JsonOwnedLine<'a> {
    highlighted: bool,
    number: usize,
    #[serde(serialize_with = "serialize_owned_tokens")]
    tokens: &'a [comfit_core::OwnedToken],
}

fn serialize_owned_tokens<S: Serializer>(
    tokens: &[comfit_core::OwnedToken],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(tokens.iter().map(|token| JsonToken {
        highlight: token.highlight.as_deref(),
        rainbow: token.rainbow,
        text: &token.text,
    }))
}

struct JsonOwnedLines<'a>(&'a comfit_core::HighlightResult);

impl Serialize for JsonOwnedLines<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0.lines.iter().map(|line| JsonOwnedLine {
            highlighted: line.highlighted,
            number: line.number,
            tokens: &line.tokens,
        }))
    }
}

/// A language registry using web-tree-sitter in the browser.
///
/// Methods borrow the registry only for synchronous engine work. Grammar loading
/// awaits runtime initialization before taking a mutable borrow.
#[wasm_bindgen]
pub struct Highlighter {
    inner: RefCell<comfit_core::Highlighter<backend::WebBackend>>,
}

/// Initialize web-tree-sitter from WASM bytes. Concurrent calls share initialization.
///
/// # Errors
///
/// Rejects the promise when runtime initialization fails. Failed initialization
/// can be retried.
#[wasm_bindgen]
pub async fn initialize(tree_sitter_wasm: Uint8Array) -> Result<(), JsValue> {
    binding::TreeSitter::init(tree_sitter_wasm).await
}

/// Create an empty highlighter after runtime initialization.
///
/// # Errors
///
/// Returns a JavaScript error if the runtime cannot create a parser.
#[wasm_bindgen]
pub fn create_highlighter() -> Result<Highlighter, JsValue> {
    let backend = backend::WebBackend::new().map_err(js_error)?;

    Ok(Highlighter {
        inner: RefCell::new(comfit_core::Highlighter::new(backend)),
    })
}

#[wasm_bindgen]
impl Highlighter {
    /// Create a document sharing the currently loaded language queries.
    pub fn create_document(&self, language: &str, source: &str) -> Result<Document, JsValue> {
        let backend = backend::WebBackend::new().map_err(js_error)?;
        let inner = self
            .inner
            .try_borrow()
            .map_err(js_error)?
            .create_document(language, source, backend)
            .map_err(js_error)?;
        Ok(Document { inner })
    }

    /// Load a grammar from bytes and register its JSON language definition.
    ///
    /// # Errors
    ///
    /// Rejects invalid JSON, grammar loading failures, duplicate names, query failures,
    /// or a conflicting registry borrow.
    pub async fn load_language(
        &self,
        grammar: Uint8Array,
        definition: &str,
    ) -> Result<(), JsValue> {
        let definition: LanguageDefinition = serde_json::from_str(definition).map_err(js_error)?;
        let language = binding::Language::load_bytes(&grammar).await?;
        let aliases = definition
            .aliases
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        self.inner
            .try_borrow_mut()
            .map_err(js_error)?
            .load(
                &definition.id,
                &aliases,
                language,
                Queries {
                    highlights: &definition.highlights,
                    injections: &definition.injections,
                    locals: &definition.locals,
                    rainbows: &definition.rainbows,
                },
            )
            .map_err(js_error)
    }

    /// Highlight source and render escaped class, inline, or variable-based HTML.
    ///
    /// Inline themes are supplied as validated JSON objects. Source parsing is shared
    /// by all palettes. Theme switching uses caller-provided CSS, not generated scripts.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error for invalid options, a conflicting registry borrow,
    /// or a highlighting failure.
    pub fn render_html(
        &self,
        source: &str,
        language: &str,
        options: &str,
    ) -> Result<String, JsValue> {
        let options: Options = serde_json::from_str(options).map_err(js_error)?;
        let config = options.render().map_err(js_error)?;
        let mut inner = self.inner.try_borrow_mut().map_err(js_error)?;
        let highlighted = inner
            .highlight(source, language, &options.highlight())
            .map_err(js_error)?;

        render_highlighted(&highlighted, &options, &config).map_err(js_error)
    }

    /// Highlight source and serialize its lines and tokens, borrowing token text.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error for invalid options, a conflicting registry borrow,
    /// highlighting failures, or serialization failures.
    pub fn highlight_json(
        &self,
        source: &str,
        language: &str,
        options: &str,
    ) -> Result<String, JsValue> {
        let options: Options = serde_json::from_str(options).map_err(js_error)?;
        let mut inner = self.inner.try_borrow_mut().map_err(js_error)?;
        let highlighted = inner
            .highlight(source, language, &options.highlight())
            .map_err(js_error)?;
        lines_json(&highlighted)
    }
}

fn js_error(error: impl std::fmt::Display) -> JsValue {
    js_sys::Error::new(&error.to_string()).into()
}

#[cfg(test)]
mod tests {
    use comfit_core::{Highlighted, Line, Token};

    use super::lines_json;

    #[test]
    fn borrowed_json_matches_the_existing_wire_format() {
        let highlighted = Highlighted {
            lines: vec![
                Line {
                    highlighted: true,
                    number: 1,
                    tokens: vec![
                        Token {
                            highlight: Some("keyword.function"),
                            rainbow: None,
                            text: "café 😀\n\"<&",
                        },
                        Token {
                            highlight: None,
                            rainbow: Some(0),
                            text: "[",
                        },
                    ],
                },
                Line {
                    highlighted: false,
                    number: 2,
                    tokens: vec![],
                },
            ],
        };
        let previous = serde_json::json!([
            {
                "number": 1, "highlighted": true,
                "tokens": [
                    { "text": "café 😀\n\"<&", "highlight": "keyword.function", "rainbow": null },
                    { "text": "[", "highlight": null, "rainbow": 0 },
                ],
            },
            { "number": 2, "highlighted": false, "tokens": [] },
        ]);

        assert_eq!(
            lines_json(&highlighted).unwrap(),
            serde_json::to_string(&previous).unwrap(),
        );
    }
}

fn render_highlighted(
    highlighted: &comfit_core::Highlighted<'_>,
    options: &Options,
    config: &RenderHtmlConfig,
) -> Result<String, Error> {
    if config.style != RenderHtmlStyle::Inline {
        return Ok(highlighted.render_html_classes(config));
    }

    let themes = named_themes(options)?;
    highlighted.render_html_themes(&themes.palettes, themes.default, config)
}

struct NamedThemes<'a> {
    palettes: Vec<(&'a str, &'a comfit_core::Theme)>,
    default: &'a str,
}

fn named_themes(options: &Options) -> Result<NamedThemes<'_>, Error> {
    if let Some(theme) = &options.theme {
        return Ok(NamedThemes {
            palettes: vec![("theme", &theme.0)],
            default: "theme",
        });
    }
    let themes: Vec<_> = options
        .themes
        .as_ref()
        .into_iter()
        .flat_map(|themes| themes.iter())
        .map(|(name, theme)| (name.as_str(), &theme.0))
        .collect();
    let default = options
        .default_theme
        .as_deref()
        .or_else(|| themes.first().map(|(name, _)| *name))
        .ok_or_else(|| Error::Html("inline style requires a theme".into()))?;
    Ok(NamedThemes {
        palettes: themes,
        default,
    })
}

struct JsonLines<'a, 'source>(&'a comfit_core::Highlighted<'source>);

impl Serialize for JsonLines<'_, '_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0.lines.iter().map(|line| JsonLine {
            highlighted: line.highlighted,
            number: line.number,
            tokens: &line.tokens,
        }))
    }
}

fn lines_json(highlighted: &comfit_core::Highlighted<'_>) -> Result<String, JsValue> {
    serde_json::to_string(&JsonLines(highlighted)).map_err(js_error)
}

/// An incremental source document that remains valid after its original registry is dropped.
#[wasm_bindgen]
pub struct Document {
    inner: comfit_core::Document<backend::WebBackend>,
}

/// An owned highlighting snapshot. Call free() when it is no longer needed.
#[wasm_bindgen]
pub struct HighlightResult {
    inner: comfit_core::HighlightResult,
}

#[wasm_bindgen]
impl Document {
    pub fn source(&self) -> String {
        self.inner.source().to_owned()
    }

    pub fn update(&mut self, source: &str) -> Result<(), JsValue> {
        self.inner.update(source).map_err(js_error)
    }

    pub fn highlight(&mut self, options: &str) -> Result<HighlightResult, JsValue> {
        let options: Options = serde_json::from_str(options).map_err(js_error)?;
        Ok(HighlightResult {
            inner: self
                .inner
                .highlight(&options.highlight())
                .map_err(js_error)?,
        })
    }
}

#[wasm_bindgen]
impl HighlightResult {
    pub fn line_count(&self) -> usize {
        self.inner.lines.len()
    }

    pub fn lines_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&JsonOwnedLines(&self.inner)).map_err(js_error)
    }

    pub fn render_html(&self, options: &str) -> Result<String, JsValue> {
        let options: Options = serde_json::from_str(options).map_err(js_error)?;
        let config = options.render().map_err(js_error)?;
        if let Some(lines) = &options.highlighted_lines {
            let highlighted = self.inner.with_highlighted_lines(lines);
            return render_highlighted(&highlighted, &options, &config).map_err(js_error);
        }
        if config.style != RenderHtmlStyle::Inline {
            return Ok(self.inner.render_html_classes(&config));
        }
        let themes = named_themes(&options).map_err(js_error)?;
        self.inner
            .render_html_themes(&themes.palettes, themes.default, &config)
            .map_err(js_error)
    }
}
