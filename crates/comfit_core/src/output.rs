//! Borrowed highlighting results and owned snapshots.

use smol_str::SmolStr;

/// Highlighting options used by the engine and renderers.
#[derive(Debug, Clone)]
pub struct HighlightConfig {
    /// One-based source lines to emphasize.
    pub highlighted_lines: Vec<usize>,
    /// Collect bracket nesting depths from the language rainbow query. Defaults to `true`.
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

/// Highlighted lines borrowing source text and query capture names.
///
/// Even an empty source produces one line. A trailing newline produces a final
/// empty line. Tokens exclude newline separators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Highlighted<'a> {
    /// Lines in source order, starting at line one.
    pub lines: Vec<Line<'a>>,
}

/// A source line and its tokens, without its newline separator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line<'a> {
    /// One-based source line number.
    pub number: usize,
    /// Whether the caller selected this line for emphasis.
    pub highlighted: bool,
    /// Tokens in source order. Token boundaries may differ from syntax node boundaries.
    pub tokens: Vec<Token<'a>>,
}

/// A borrowed source slice with an optional capture style and bracket depth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    /// Source text, without newline separators.
    pub text: &'a str,
    /// Query capture name, or `None` for unstyled text.
    pub highlight: Option<&'a str>,
    /// Zero-based nesting depth, independent of the rendering palette.
    pub rainbow: Option<usize>,
}

/// An owned highlighting snapshot that remains valid after its document changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightResult {
    /// Lines in source order, including a final empty line after a trailing newline.
    pub lines: Vec<OwnedLine>,
}

/// An owned source line without its newline separator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedLine {
    /// One-based source line number.
    pub number: usize,
    /// Whether the caller selected this line for emphasis.
    pub highlighted: bool,
    /// Tokens in source order, without newline separators.
    pub tokens: Vec<OwnedToken>,
}

/// Owned token text, capture name, and optional bracket depth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedToken {
    /// Source text, without newline separators.
    pub text: String,
    /// Query capture name, or `None` for unstyled text.
    pub highlight: Option<SmolStr>,
    /// Zero-based nesting depth, independent of the rendering palette.
    pub rainbow: Option<usize>,
}

impl Highlighted<'_> {
    /// Copy this borrowed output into an independent snapshot.
    pub fn to_owned(&self) -> HighlightResult {
        HighlightResult {
            lines: self
                .lines
                .iter()
                .map(|line| OwnedLine {
                    number: line.number,
                    highlighted: line.highlighted,
                    tokens: line
                        .tokens
                        .iter()
                        .map(|token| OwnedToken {
                            text: token.text.to_owned(),
                            highlight: token.highlight.map(SmolStr::new),
                            rainbow: token.rainbow,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

impl HighlightResult {
    /// Create a borrowed view of this snapshot without parsing or running queries.
    ///
    /// The view borrows token strings and allocates new line and token vectors.
    /// The view cannot outlive this snapshot.
    pub fn as_highlighted(&self) -> Highlighted<'_> {
        Highlighted {
            lines: self
                .lines
                .iter()
                .map(|line| Line {
                    number: line.number,
                    highlighted: line.highlighted,
                    tokens: line
                        .tokens
                        .iter()
                        .map(|token| Token {
                            text: &token.text,
                            highlight: token.highlight.as_deref(),
                            rainbow: token.rainbow,
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    /// Create a borrowed view with different line emphasis, without re-highlighting.
    pub fn with_highlighted_lines(&self, lines: &[usize]) -> Highlighted<'_> {
        let mut highlighted = self.as_highlighted();
        let mut selected = lines.to_vec();
        selected.sort_unstable();
        let mut selected = selected.into_iter().peekable();

        for line in &mut highlighted.lines {
            while selected.peek().is_some_and(|number| *number < line.number) {
                selected.next();
            }
            line.highlighted = selected.peek() == Some(&line.number);
        }
        highlighted
    }

    #[cfg(feature = "render-html")]
    pub fn render_html(&self, theme: &crate::Theme, config: &crate::RenderHtmlConfig) -> String {
        crate::render::html::render(self.lines.iter(), Some(theme), &[], config, false)
    }

    #[cfg(feature = "render-html")]
    pub fn render_html_classes(&self, config: &crate::RenderHtmlConfig) -> String {
        crate::render::html::render(self.lines.iter(), None, &[], config, false)
    }

    #[cfg(feature = "render-html")]
    pub fn render_html_themes(
        &self,
        themes: &[(&str, &crate::Theme)],
        default_theme: &str,
        config: &crate::RenderHtmlConfig,
    ) -> Result<String, crate::Error> {
        crate::render::html::render_themes(self.lines.iter(), themes, default_theme, config)
    }

    #[cfg(feature = "render-ansi")]
    pub fn render_ansi(&self, theme: &crate::Theme) -> String {
        crate::render::ansi::render(self.lines.iter(), theme)
    }
}
