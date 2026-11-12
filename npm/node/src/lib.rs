//! Synchronous Node-API bindings with a cached highlighter per Node worker thread.

use std::cell::RefCell;

use comfit::{Error, HighlightConfig as CoreHighlightConfig, RenderHtmlConfig as CoreHtmlConfig};
use napi_derive::napi;

/// How token styles are rendered in HTML, with or without a theme.
#[napi]
#[derive(Clone, Copy, Default)]
pub enum RenderHtmlStyle {
    /// Embed literal theme styles in each token element.
    Inline,
    /// Emit capture classes for a caller-provided stylesheet.
    #[default]
    Class,
    /// Reference caller-provided CSS variables instead of theme values.
    Variables,
}

/// Options for token output and ANSI rendering. Line numbers are one-based.
#[napi]
pub struct HighlightConfig {
    /// One-based source lines to emphasize. Defaults to an empty list.
    pub highlighted_lines: Vec<f64>,
    /// Collect bracket depths. Defaults to `true`.
    pub rainbow: bool,
}

impl Default for HighlightConfig {
    fn default() -> Self {
        Self {
            highlighted_lines: Vec::new(),
            rainbow: true,
        }
    }
}

#[napi]
impl HighlightConfig {
    /// Create highlighting options with bracket-depth collection enabled.
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl HighlightConfig {
    fn core(&self) -> napi::Result<CoreHighlightConfig> {
        Ok(CoreHighlightConfig {
            highlighted_lines: self
                .highlighted_lines
                .iter()
                .map(|&line| index(line, "highlightedLines"))
                .collect::<napi::Result<_>>()?,
            rainbow: self.rainbow,
        })
    }
}

/// Highlighting and HTML options. Defaults to capture classes and bracket-depth collection.
#[napi]
pub struct RenderHtmlConfig {
    /// One-based source lines to emphasize. Defaults to an empty list.
    pub highlighted_lines: Vec<f64>,
    /// Collect bracket depths. Defaults to `true`.
    pub rainbow: bool,
    /// Outer tag name. Defaults to `pre`.
    pub pre_tag: String,
    /// Line tag name. Defaults to `span`.
    pub line_tag: String,
    /// Optional class on the outer element, escaped as attribute text.
    pub pre_class: Option<String>,
    /// Prefix for token, rainbow, and line classes. Defaults to an empty string.
    pub class_prefix: String,
    /// Rainbow wrapping count. Zero preserves depth when no theme palette applies.
    pub rainbow_colors: f64,
    /// How token styles are rendered. Defaults to [`RenderHtmlStyle::Class`].
    pub style: RenderHtmlStyle,
}

impl Default for RenderHtmlConfig {
    fn default() -> Self {
        Self {
            highlighted_lines: Vec::new(),
            rainbow: true,
            pre_tag: "pre".into(),
            line_tag: "span".into(),
            pre_class: None,
            class_prefix: String::new(),
            rainbow_colors: 0.0,
            style: RenderHtmlStyle::Class,
        }
    }
}

#[napi]
impl RenderHtmlConfig {
    /// Create HTML options with capture classes and bracket-depth collection enabled.
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl RenderHtmlConfig {
    fn highlight(&self) -> napi::Result<CoreHighlightConfig> {
        Ok(CoreHighlightConfig {
            highlighted_lines: self
                .highlighted_lines
                .iter()
                .map(|&line| index(line, "highlightedLines"))
                .collect::<napi::Result<_>>()?,
            rainbow: self.rainbow,
        })
    }

    fn html(&self) -> napi::Result<CoreHtmlConfig> {
        Ok(CoreHtmlConfig {
            pre_tag: self.pre_tag.clone(),
            line_tag: self.line_tag.clone(),
            pre_class: self.pre_class.clone(),
            class_prefix: self.class_prefix.clone(),
            rainbow_colors: index(self.rainbow_colors, "rainbowColors")?,
            style: match self.style {
                RenderHtmlStyle::Inline => comfit::RenderHtmlStyle::Inline,
                RenderHtmlStyle::Class => comfit::RenderHtmlStyle::Class,
                RenderHtmlStyle::Variables => comfit::RenderHtmlStyle::Variables,
            },
        })
    }
}

/// An owned source slice with an optional capture and zero-based bracket depth.
#[napi(object, use_nullable = true)]
pub struct Token {
    /// Source text without newline separators.
    pub text: String,
    /// Capture name, or `null` for unstyled text.
    pub highlight: Option<String>,
    /// Zero-based bracket depth, or `null` if none was collected.
    pub rainbow: Option<u32>,
}

/// One source line without its newline separator. Tokens remain in source order.
#[napi(object)]
pub struct Line {
    /// One-based source line number.
    pub number: u32,
    /// Whether this line is selected for emphasis.
    pub highlighted: bool,
    /// Tokens in source order. Token boundaries may differ from syntax node boundaries.
    pub tokens: Vec<Token>,
}

thread_local! {
    static HIGHLIGHTER: RefCell<Option<comfit::Highlighter>> = const { RefCell::new(None) };
}

fn with_highlighter<T>(
    render: impl FnOnce(&mut comfit::Highlighter) -> napi::Result<T>,
) -> napi::Result<T> {
    HIGHLIGHTER.with(|cell| {
        let mut cached = cell.try_borrow_mut().map_err(js_error)?;
        if cached.is_none() {
            *cached = Some(comfit::Highlighter::new().map_err(core_error)?);
        }
        render(cached.as_mut().expect("highlighter initialized above"))
    })
}

fn theme(id: &str) -> napi::Result<&'static comfit::Theme> {
    comfit_themes::THEMES
        .iter()
        .find(|theme| theme.id == id)
        .map(|definition| (definition.theme)())
        .ok_or_else(|| napi::Error::new(napi::Status::InvalidArg, format!("unknown theme: {id}")))
}

fn index(value: f64, field: &str) -> napi::Result<usize> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 || value > 9_007_199_254_740_991.0
    {
        return Err(napi::Error::new(
            napi::Status::InvalidArg,
            format!("{field}: expected a nonnegative safe integer"),
        ));
    }
    usize::try_from(value as u64).map_err(|_| {
        napi::Error::new(
            napi::Status::InvalidArg,
            format!("{field}: integer exceeds this platform's range"),
        )
    })
}

fn js_error(error: impl std::fmt::Display) -> napi::Error {
    napi::Error::from_reason(error.to_string())
}

fn core_error(error: Error) -> napi::Error {
    let status = match &error {
        Error::Backend(_) | Error::InjectionDepth => napi::Status::GenericFailure,
        _ => napi::Status::InvalidArg,
    };
    napi::Error::new(status, error.to_string())
}

/// Highlight source into owned lines and tokens.
///
/// Calls are synchronous. Use a Node worker for large inputs. Results remain usable after later calls.
///
/// # Errors
///
/// Returns `InvalidArg` for unknown languages, invalid ranges, or invalid numeric options.
/// Backend failures, excessive injection nesting, and output-index overflows return `GenericFailure`.
#[napi]
pub fn highlight(
    source: String,
    language: String,
    config: Option<&HighlightConfig>,
) -> napi::Result<Vec<Line>> {
    let config = config
        .map(HighlightConfig::core)
        .transpose()?
        .unwrap_or_default();
    with_highlighter(|engine| {
        let highlighted = engine
            .highlight(&source, &language, &config)
            .map_err(core_error)?;
        to_lines(highlighted)
    })
}

/// Render escaped HTML with a bundled theme.
///
/// Class styles need a stylesheet. Variable styles need caller-provided CSS variables.
///
/// # Errors
///
/// Returns `InvalidArg` for invalid themes, languages, source ranges, or numeric options.
/// Backend failures and excessive injection nesting return `GenericFailure`.
#[napi]
pub fn render_html(
    source: String,
    language: String,
    theme: String,
    config: Option<&RenderHtmlConfig>,
) -> napi::Result<String> {
    let theme = self::theme(&theme)?;
    let highlight = config
        .map(RenderHtmlConfig::highlight)
        .transpose()?
        .unwrap_or_default();
    let html = config
        .map(RenderHtmlConfig::html)
        .transpose()?
        .unwrap_or_default();
    with_highlighter(|engine| {
        Ok(engine
            .highlight(&source, &language, &highlight)
            .map_err(core_error)?
            .render_html(theme, &html))
    })
}

/// Render escaped HTML without a theme.
///
/// Class and variable styles use caller-provided CSS. Inline style falls back to classes.
///
/// # Errors
///
/// Returns `InvalidArg` for invalid languages, source ranges, or numeric options.
/// Backend failures and excessive injection nesting return `GenericFailure`.
#[napi]
pub fn render_html_classes(
    source: String,
    language: String,
    config: Option<&RenderHtmlConfig>,
) -> napi::Result<String> {
    let highlight = config
        .map(RenderHtmlConfig::highlight)
        .transpose()?
        .unwrap_or_default();
    let html = config
        .map(RenderHtmlConfig::html)
        .transpose()?
        .unwrap_or_default();

    with_highlighter(|engine| {
        Ok(engine
            .highlight(&source, &language, &highlight)
            .map_err(core_error)?
            .render_html_classes(&html))
    })
}

/// A bundled theme ID and its CSS variable namespace.
#[napi(object)]
pub struct NamedTheme {
    /// CSS namespace containing ASCII letters, digits, underscores, or hyphens.
    pub name: String,
    /// Bundled theme ID, as returned by [`themes`].
    pub theme: String,
}

/// Render inline HTML with a default theme and named alternative themes.
///
/// Alternatives use `--comfit-<name>` CSS properties. Caller CSS activates them.
/// Omitted configuration defaults to inline style.
///
/// # Errors
///
/// Returns `InvalidArg` for invalid themes, names, default selection, non-inline style,
/// languages, source ranges, or numeric options. Backend failures and excessive
/// injection nesting return `GenericFailure`.
#[napi]
pub fn render_html_themes(
    source: String,
    language: String,
    themes: Vec<NamedTheme>,
    default_theme: String,
    config: Option<&RenderHtmlConfig>,
) -> napi::Result<String> {
    let highlight = config
        .map(RenderHtmlConfig::highlight)
        .transpose()?
        .unwrap_or_default();
    let html = config
        .map(RenderHtmlConfig::html)
        .transpose()?
        .unwrap_or_else(|| CoreHtmlConfig {
            style: comfit::RenderHtmlStyle::Inline,
            ..CoreHtmlConfig::default()
        });
    let themes = themes
        .iter()
        .map(|named| Ok((named.name.as_str(), theme(&named.theme)?)))
        .collect::<napi::Result<Vec<_>>>()?;

    with_highlighter(|engine| {
        engine
            .highlight(&source, &language, &highlight)
            .map_err(core_error)?
            .render_html_themes(&themes, &default_theme, &html)
            .map_err(core_error)
    })
}

/// Render ANSI-styled source with a bundled theme.
///
/// The core renderer escapes controls other than tabs and newlines. It does not add
/// CLI gutters, wrapping, padding, or color detection.
///
/// # Errors
///
/// Returns `InvalidArg` for invalid themes, languages, source ranges, or numeric options.
/// Backend failures and excessive injection nesting return `GenericFailure`.
#[napi]
pub fn render_ansi(
    source: String,
    language: String,
    theme: String,
    config: Option<&HighlightConfig>,
) -> napi::Result<String> {
    let theme = self::theme(&theme)?;
    let config = config
        .map(HighlightConfig::core)
        .transpose()?
        .unwrap_or_default();
    with_highlighter(|engine| {
        Ok(engine
            .highlight(&source, &language, &config)
            .map_err(core_error)?
            .render_ansi(theme))
    })
}

/// Return whether a bundled language ID or alias is available.
///
/// # Errors
///
/// Returns an error if the cached highlighter cannot be initialized or borrowed.
#[napi]
pub fn contains_language(language: String) -> napi::Result<bool> {
    with_highlighter(|engine| Ok(engine.contains(&language)))
}

/// Return bundled language IDs. Language aliases are also accepted by rendering functions.
#[napi]
pub fn languages() -> Vec<&'static str> {
    comfit::grammars::LANGUAGES
        .iter()
        .map(|language| language.id)
        .collect()
}

/// Return bundled theme IDs without initializing their palettes.
#[napi]
pub fn themes() -> Vec<&'static str> {
    comfit_themes::THEMES.iter().map(|theme| theme.id).collect()
}

fn to_lines(highlighted: comfit::Highlighted<'_>) -> napi::Result<Vec<Line>> {
    highlighted
        .lines
        .into_iter()
        .map(|line| {
            Ok(Line {
                number: u32::try_from(line.number).map_err(js_error)?,
                highlighted: line.highlighted,
                tokens: line
                    .tokens
                    .into_iter()
                    .map(|token| {
                        Ok(Token {
                            text: token.text.to_owned(),
                            highlight: token.highlight.map(str::to_owned),
                            rainbow: token
                                .rainbow
                                .map(u32::try_from)
                                .transpose()
                                .map_err(js_error)?,
                        })
                    })
                    .collect::<napi::Result<_>>()?,
            })
        })
        .collect()
}

/// Initial source and a bundled language ID or alias for a document session.
#[napi(object)]
pub struct DocumentOptions {
    pub language: String,
    pub source: String,
}

/// An incremental source document. Call dispose() when it is no longer needed.
#[napi]
pub struct Document {
    inner: Option<comfit::Document>,
}

/// An owned snapshot that remains usable after document edits or disposal.
#[napi]
pub struct HighlightResult {
    inner: comfit::HighlightResult,
}

/// Create a document sharing the worker's cached grammars and compiled queries.
#[napi]
pub fn create_document(options: DocumentOptions) -> napi::Result<Document> {
    with_highlighter(|engine| {
        Ok(Document {
            inner: Some(
                engine
                    .create_document(&options.language, &options.source)
                    .map_err(core_error)?,
            ),
        })
    })
}

#[napi]
impl Document {
    #[napi(getter)]
    pub fn source(&self) -> napi::Result<String> {
        Ok(self
            .inner
            .as_ref()
            .ok_or_else(|| js_error("Document has been disposed"))?
            .source()
            .to_owned())
    }

    /// Incrementally parse changed source. If the source is unchanged, do nothing.
    #[napi]
    pub fn update(&mut self, source: String) -> napi::Result<()> {
        self.inner
            .as_mut()
            .ok_or_else(|| js_error("Document has been disposed"))?
            .update(&source)
            .map_err(core_error)
    }

    /// Return an owned snapshot. Changes to line emphasis reuse cached highlighting.
    #[napi]
    pub fn highlight(&mut self, config: Option<&HighlightConfig>) -> napi::Result<HighlightResult> {
        let config = config
            .map(HighlightConfig::core)
            .transpose()?
            .unwrap_or_default();
        Ok(HighlightResult {
            inner: self
                .inner
                .as_mut()
                .ok_or_else(|| js_error("Document has been disposed"))?
                .highlight(&config)
                .map_err(core_error)?,
        })
    }

    /// Release retained trees and language references. Repeated calls are harmless.
    #[napi]
    pub fn dispose(&mut self) {
        self.inner = None;
    }
}

impl HighlightResult {
    fn highlighted(
        &self,
        config: Option<&RenderHtmlConfig>,
    ) -> napi::Result<comfit::Highlighted<'_>> {
        Ok(if let Some(config) = config {
            self.inner
                .with_highlighted_lines(&config.highlight()?.highlighted_lines)
        } else {
            self.inner.as_highlighted()
        })
    }
}

#[napi]
impl HighlightResult {
    /// Copy the snapshot's lines and tokens into JavaScript-owned values.
    #[napi(getter)]
    pub fn lines(&self) -> napi::Result<Vec<Line>> {
        self.inner
            .lines
            .iter()
            .map(|line| {
                Ok(Line {
                    number: u32::try_from(line.number).map_err(js_error)?,
                    highlighted: line.highlighted,
                    tokens: line
                        .tokens
                        .iter()
                        .map(|token| {
                            Ok(Token {
                                text: token.text.clone(),
                                highlight: token.highlight.as_deref().map(str::to_owned),
                                rainbow: token
                                    .rainbow
                                    .map(u32::try_from)
                                    .transpose()
                                    .map_err(js_error)?,
                            })
                        })
                        .collect::<napi::Result<_>>()?,
                })
            })
            .collect()
    }

    /// Render a snapshot without parsing. Bracket depths are set by highlight().
    #[napi]
    pub fn render_html(
        &self,
        theme: String,
        config: Option<&RenderHtmlConfig>,
    ) -> napi::Result<String> {
        let theme = self::theme(&theme)?;
        let html = config
            .map(RenderHtmlConfig::html)
            .transpose()?
            .unwrap_or_default();
        Ok(if config.is_none() {
            self.inner.render_html(theme, &html)
        } else {
            self.highlighted(config)?.render_html(theme, &html)
        })
    }

    #[napi]
    pub fn render_html_classes(&self, config: Option<&RenderHtmlConfig>) -> napi::Result<String> {
        let html = config
            .map(RenderHtmlConfig::html)
            .transpose()?
            .unwrap_or_default();
        Ok(if config.is_none() {
            self.inner.render_html_classes(&html)
        } else {
            self.highlighted(config)?.render_html_classes(&html)
        })
    }

    #[napi]
    pub fn render_html_themes(
        &self,
        themes: Vec<NamedTheme>,
        default_theme: String,
        config: Option<&RenderHtmlConfig>,
    ) -> napi::Result<String> {
        let html = config
            .map(RenderHtmlConfig::html)
            .transpose()?
            .unwrap_or_else(|| CoreHtmlConfig {
                style: comfit::RenderHtmlStyle::Inline,
                ..Default::default()
            });
        let themes = themes
            .iter()
            .map(|named| Ok((named.name.as_str(), theme(&named.theme)?)))
            .collect::<napi::Result<Vec<_>>>()?;
        if config.is_none() {
            self.inner
                .render_html_themes(&themes, &default_theme, &html)
        } else {
            self.highlighted(config)?
                .render_html_themes(&themes, &default_theme, &html)
        }
        .map_err(core_error)
    }

    #[napi]
    pub fn render_ansi(&self, theme: String) -> napi::Result<String> {
        Ok(self.inner.render_ansi(self::theme(&theme)?))
    }
}
