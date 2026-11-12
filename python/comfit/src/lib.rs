//! Python bindings with a lazily initialized highlighter per calling thread.

use std::cell::RefCell;

use comfit::{Error, HighlightConfig as CoreHighlightConfig, RenderHtmlConfig as CoreHtmlConfig};
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};

/// How token styles are rendered in HTML. Rendering always requires an explicit theme ID.
#[pyclass(module = "comfit", eq, eq_int, from_py_object)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum RenderHtmlStyle {
    /// Embed literal theme styles in each token element.
    Inline,
    /// Emit capture classes for a caller-provided stylesheet.
    #[default]
    Class,
    /// Reference caller-provided CSS variables instead of theme values.
    Variables,
}

/// Options for token output and ANSI rendering. Line numbers are one-based.
#[pyclass(module = "comfit", get_all, set_all)]
struct HighlightConfig {
    /// One-based source lines to emphasize. Defaults to an empty list.
    highlighted_lines: Vec<usize>,
    /// Collect bracket depths. Defaults to True.
    rainbow: bool,
}

#[pymethods]
impl HighlightConfig {
    /// Create highlighting options. Bracket-depth collection is enabled by default.
    #[new]
    #[pyo3(signature = (*, highlighted_lines=None, rainbow=true))]
    fn new(highlighted_lines: Option<Vec<usize>>, rainbow: bool) -> Self {
        Self {
            highlighted_lines: highlighted_lines.unwrap_or_default(),
            rainbow,
        }
    }
}

impl HighlightConfig {
    fn core(&self) -> CoreHighlightConfig {
        CoreHighlightConfig {
            highlighted_lines: self.highlighted_lines.clone(),
            rainbow: self.rainbow,
        }
    }
}

/// Highlighting and HTML options. Defaults to capture classes and bracket-depth collection.
#[pyclass(module = "comfit", get_all, set_all)]
struct RenderHtmlConfig {
    /// One-based source lines to emphasize. Defaults to an empty list.
    highlighted_lines: Vec<usize>,
    /// Collect bracket depths. Defaults to True.
    rainbow: bool,
    /// Outer tag name. Defaults to `pre`.
    pre_tag: String,
    /// Line tag name. Defaults to `span`.
    line_tag: String,
    /// Optional class on the outer element, escaped as attribute text.
    pre_class: Option<String>,
    /// Prefix for token, rainbow, and line classes. Defaults to an empty string.
    class_prefix: String,
    /// Rainbow wrapping count. Zero preserves depth when no theme palette applies.
    rainbow_colors: usize,
    /// How token styles are rendered. Defaults to RenderHtmlStyle.Class.
    style: RenderHtmlStyle,
}

#[pymethods]
impl RenderHtmlConfig {
    /// Create HTML options. Capture classes and bracket-depth collection are enabled by default.
    #[new]
    #[pyo3(signature = (*, highlighted_lines=None, rainbow=true, pre_class=None, class_prefix=String::new(), rainbow_colors=0, style=RenderHtmlStyle::Class, pre_tag="pre", line_tag="span"))]
    #[allow(
        clippy::too_many_arguments,
        reason = "Each HTML option is a Python keyword argument."
    )]
    fn new(
        highlighted_lines: Option<Vec<usize>>,
        rainbow: bool,
        pre_class: Option<String>,
        class_prefix: String,
        rainbow_colors: usize,
        style: RenderHtmlStyle,
        pre_tag: &str,
        line_tag: &str,
    ) -> Self {
        Self {
            highlighted_lines: highlighted_lines.unwrap_or_default(),
            rainbow,
            pre_tag: pre_tag.to_owned(),
            line_tag: line_tag.to_owned(),
            pre_class,
            class_prefix,
            rainbow_colors,
            style,
        }
    }
}

impl RenderHtmlConfig {
    fn highlight(&self) -> CoreHighlightConfig {
        CoreHighlightConfig {
            highlighted_lines: self.highlighted_lines.clone(),
            rainbow: self.rainbow,
        }
    }

    fn html(&self) -> CoreHtmlConfig {
        CoreHtmlConfig {
            pre_tag: self.pre_tag.clone(),
            line_tag: self.line_tag.clone(),
            pre_class: self.pre_class.clone(),
            class_prefix: self.class_prefix.clone(),
            rainbow_colors: self.rainbow_colors,
            style: match self.style {
                RenderHtmlStyle::Inline => comfit::RenderHtmlStyle::Inline,
                RenderHtmlStyle::Class => comfit::RenderHtmlStyle::Class,
                RenderHtmlStyle::Variables => comfit::RenderHtmlStyle::Variables,
            },
        }
    }
}

/// A source slice copied into Python, with optional capture and bracket depth.
#[pyclass(module = "comfit", frozen, get_all, from_py_object)]
#[derive(Clone)]
struct Token {
    /// Source text without newline separators.
    text: String,
    /// Capture name, or None for unstyled text.
    highlight: Option<String>,
    /// Zero-based bracket depth, or None if none was collected.
    rainbow: Option<usize>,
}

/// One source line without its newline separator. Tokens remain in source order.
#[pyclass(module = "comfit", frozen, get_all, from_py_object)]
#[derive(Clone)]
struct Line {
    /// One-based source line number.
    number: usize,
    /// Whether this line is selected for emphasis.
    highlighted: bool,
    /// Tokens in source order. Token boundaries may differ from syntax node boundaries.
    tokens: Vec<Token>,
}

thread_local! {
    static HIGHLIGHTER: RefCell<Option<comfit::Highlighter>> = const { RefCell::new(None) };
}

fn with_highlighter<T>(
    render: impl FnOnce(&mut comfit::Highlighter) -> Result<T, Error>,
) -> Result<T, Error> {
    HIGHLIGHTER.with(|cell| {
        let mut cached = cell
            .try_borrow_mut()
            .map_err(|_| Error::Backend("highlighter is already in use".into()))?;
        if cached.is_none() {
            *cached = Some(comfit::Highlighter::new()?);
        }
        render(cached.as_mut().expect("highlighter initialized above"))
    })
}

fn theme(id: &str) -> PyResult<&'static comfit::Theme> {
    comfit_themes::THEMES
        .iter()
        .find(|theme| theme.id == id)
        .map(|definition| (definition.theme)())
        .ok_or_else(|| PyValueError::new_err(format!("unknown theme: {id}")))
}

fn py_error(error: Error) -> PyErr {
    let message = error.to_string();
    match error {
        Error::Backend(_) | Error::InjectionDepth => PyRuntimeError::new_err(message),
        _ => PyValueError::new_err(message),
    }
}

/// Highlight source into owned lines and tokens.
///
/// Parsing and output preparation release the GIL. Results remain usable after later calls.
///
/// Raises:
///     ValueError: The language is unknown or a source range is invalid.
///     RuntimeError: The backend fails or injection nesting exceeds 32 layers.
#[pyfunction]
#[pyo3(signature = (source, language, config=None))]
fn highlight(
    py: Python<'_>,
    source: &str,
    language: &str,
    config: Option<PyRef<'_, HighlightConfig>>,
) -> PyResult<Vec<Line>> {
    let config = config
        .as_ref()
        .map(|config| config.core())
        .unwrap_or_default();
    py.detach(|| {
        with_highlighter(|engine| {
            let highlighted = engine.highlight(source, language, &config)?;
            Ok(to_lines(highlighted))
        })
    })
    .map_err(py_error)
}

/// Render escaped HTML with a bundled theme.
///
/// Class styles need a stylesheet. Variable styles need caller-provided CSS variables.
/// Parsing and rendering release the GIL.
///
/// Raises:
///     ValueError: A theme, language, or source range is invalid.
///     RuntimeError: The backend fails or injection nesting exceeds 32 layers.
#[pyfunction]
#[pyo3(signature = (source, language, theme, config=None))]
fn render_html(
    py: Python<'_>,
    source: &str,
    language: &str,
    theme: &str,
    config: Option<PyRef<'_, RenderHtmlConfig>>,
) -> PyResult<String> {
    let theme = self::theme(theme)?;
    let highlight = config
        .as_ref()
        .map(|config| config.highlight())
        .unwrap_or_default();
    let html = config
        .as_ref()
        .map(|config| config.html())
        .unwrap_or_default();
    py.detach(|| {
        with_highlighter(|engine| {
            Ok(engine
                .highlight(source, language, &highlight)?
                .render_html(theme, &html))
        })
    })
    .map_err(py_error)
}

/// Render ANSI-styled source with a bundled theme.
///
/// The core renderer escapes controls other than tabs and newlines. It does not add
/// CLI gutters, wrapping, padding, or color detection. Parsing and rendering release the GIL.
///
/// Raises:
///     ValueError: A theme, language, or source range is invalid.
///     RuntimeError: The backend fails or injection nesting exceeds 32 layers.
#[pyfunction]
#[pyo3(signature = (source, language, theme, config=None))]
fn render_ansi(
    py: Python<'_>,
    source: &str,
    language: &str,
    theme: &str,
    config: Option<PyRef<'_, HighlightConfig>>,
) -> PyResult<String> {
    let theme = self::theme(theme)?;
    let config = config
        .as_ref()
        .map(|config| config.core())
        .unwrap_or_default();
    py.detach(|| {
        with_highlighter(|engine| {
            Ok(engine
                .highlight(source, language, &config)?
                .render_ansi(theme))
        })
    })
    .map_err(py_error)
}

/// Return bundled language IDs. Aliases are also accepted by rendering functions.
#[pyfunction]
fn languages() -> Vec<&'static str> {
    comfit::grammars::LANGUAGES
        .iter()
        .map(|language| language.id)
        .collect()
}

/// Return bundled theme IDs without initializing their palettes.
#[pyfunction]
fn themes() -> Vec<&'static str> {
    comfit_themes::THEMES.iter().map(|theme| theme.id).collect()
}

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Document>()?;
    module.add_class::<HighlightResult>()?;
    module.add_function(wrap_pyfunction!(create_document, module)?)?;
    module.add_class::<RenderHtmlStyle>()?;
    module.add_class::<HighlightConfig>()?;
    module.add_class::<RenderHtmlConfig>()?;
    module.add_class::<Line>()?;
    module.add_class::<Token>()?;
    module.add_function(wrap_pyfunction!(highlight, module)?)?;
    module.add_function(wrap_pyfunction!(render_html, module)?)?;
    module.add_function(wrap_pyfunction!(render_ansi, module)?)?;
    module.add_function(wrap_pyfunction!(languages, module)?)?;
    module.add_function(wrap_pyfunction!(themes, module)?)?;
    Ok(())
}

fn to_lines(highlighted: comfit::Highlighted<'_>) -> Vec<Line> {
    highlighted
        .lines
        .into_iter()
        .map(|line| Line {
            number: line.number,
            highlighted: line.highlighted,
            tokens: line
                .tokens
                .into_iter()
                .map(|token| Token {
                    text: token.text.to_owned(),
                    highlight: token.highlight.map(str::to_owned),
                    rainbow: token.rainbow,
                })
                .collect(),
        })
        .collect()
}

/// An incremental source document with explicit disposal and context-manager support.
#[pyclass(module = "comfit")]
struct Document {
    inner: Option<comfit::Document>,
}

/// An owned snapshot that remains valid after document edits or disposal.
#[pyclass(module = "comfit", frozen)]
struct HighlightResult {
    inner: comfit::HighlightResult,
}

/// Create a document sharing the calling thread's compiled language queries.
#[pyfunction]
#[pyo3(signature = (*, language, source))]
fn create_document(py: Python<'_>, language: &str, source: &str) -> PyResult<Document> {
    py.detach(|| {
        with_highlighter(|engine| {
            Ok(Document {
                inner: Some(engine.create_document(language, source)?),
            })
        })
    })
    .map_err(py_error)
}

#[pymethods]
impl Document {
    /// Return the current source as an independent Python string.
    #[getter]
    fn source(&self) -> PyResult<&str> {
        Ok(self
            .inner
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("Document has been disposed"))?
            .source())
    }

    /// Incrementally parse changed source while releasing the GIL.
    fn update(&mut self, py: Python<'_>, source: &str) -> PyResult<()> {
        let document = self
            .inner
            .as_mut()
            .ok_or_else(|| PyRuntimeError::new_err("Document has been disposed"))?;
        py.detach(|| document.update(source)).map_err(py_error)
    }

    /// Return an owned snapshot. Changes to line emphasis reuse cached highlighting.
    #[pyo3(signature = (config=None))]
    fn highlight(
        &mut self,
        py: Python<'_>,
        config: Option<PyRef<'_, HighlightConfig>>,
    ) -> PyResult<HighlightResult> {
        let config = config
            .as_ref()
            .map(|config| config.core())
            .unwrap_or_default();
        let document = self
            .inner
            .as_mut()
            .ok_or_else(|| PyRuntimeError::new_err("Document has been disposed"))?;
        py.detach(|| {
            document
                .highlight(&config)
                .map(|inner| HighlightResult { inner })
        })
        .map_err(py_error)
    }

    /// Release retained trees and language references. Repeated calls are harmless.
    fn dispose(&mut self) {
        self.inner = None;
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        if slf.inner.is_none() {
            return Err(PyRuntimeError::new_err("Document has been disposed"));
        }
        Ok(slf)
    }

    fn __exit__(
        &mut self,
        _ty: &Bound<'_, PyAny>,
        _value: &Bound<'_, PyAny>,
        _traceback: &Bound<'_, PyAny>,
    ) {
        self.dispose();
    }
}

#[pymethods]
impl HighlightResult {
    #[getter]
    fn lines(&self) -> Vec<Line> {
        self.inner
            .lines
            .iter()
            .map(|line| Line {
                number: line.number,
                highlighted: line.highlighted,
                tokens: line
                    .tokens
                    .iter()
                    .map(|token| Token {
                        text: token.text.clone(),
                        highlight: token.highlight.as_deref().map(str::to_owned),
                        rainbow: token.rainbow,
                    })
                    .collect(),
            })
            .collect()
    }

    /// Render this snapshot without parsing. Bracket depths are set by highlight().
    #[pyo3(signature = (theme, config=None))]
    fn render_html(
        &self,
        py: Python<'_>,
        theme: &str,
        config: Option<PyRef<'_, RenderHtmlConfig>>,
    ) -> PyResult<String> {
        let theme = self::theme(theme)?;
        let html = config
            .as_ref()
            .map(|config| config.html())
            .unwrap_or_default();
        let lines = config
            .as_ref()
            .map(|config| config.highlighted_lines.clone());
        Ok(py.detach(|| {
            if let Some(lines) = lines {
                self.inner
                    .with_highlighted_lines(&lines)
                    .render_html(theme, &html)
            } else {
                self.inner.render_html(theme, &html)
            }
        }))
    }

    fn render_ansi(&self, py: Python<'_>, theme: &str) -> PyResult<String> {
        let theme = self::theme(theme)?;
        Ok(py.detach(|| self.inner.render_ansi(theme)))
    }
}
