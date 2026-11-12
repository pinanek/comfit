//! Render escaped HTML with classes, inline styles, or CSS variables.

use std::{collections::HashMap, fmt::Write};

use crate::{Error, Highlighted, Modifier, Style, Theme};

use super::RenderLine;

/// HTML elements, classes, bracket colors, and token styling options.
#[derive(Debug, Clone)]
pub struct RenderHtmlConfig {
    /// Outer tag name. Defaults to `pre`.
    pub pre_tag: String,
    /// Line tag name. Defaults to `span`.
    pub line_tag: String,
    /// Optional class attribute on the outer element.
    pub pre_class: Option<String>,
    /// Prefix for token, rainbow, and line classes, escaped as attribute text.
    pub class_prefix: String,
    /// Number of colors to cycle through for bracket classes. Zero preserves depth.
    pub rainbow_colors: usize,
    /// Style tokens with classes, inline theme values, or CSS variables. Defaults to classes.
    pub style: RenderHtmlStyle,
}

impl Default for RenderHtmlConfig {
    fn default() -> Self {
        Self {
            pre_tag: "pre".into(),
            line_tag: "span".into(),
            pre_class: None,
            class_prefix: String::new(),
            rainbow_colors: 0,
            style: RenderHtmlStyle::default(),
        }
    }
}

/// How token styles are represented in HTML.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RenderHtmlStyle {
    /// Emit capture classes for a caller-provided stylesheet.
    #[default]
    Class,
    /// Emit theme styles when a theme is supplied. Otherwise emit classes.
    Inline,
    /// Emit CSS variable references for styles defined in a stylesheet.
    Variables,
}

impl Highlighted<'_> {
    /// Render escaped HTML without a theme.
    ///
    /// Token and line classes require a caller-provided stylesheet. An inline-style
    /// configuration falls back to classes because no theme is supplied.
    /// [`RenderHtmlStyle::Variables`] emits variable references instead of theme colors.
    pub fn render_html_classes(&self, config: &RenderHtmlConfig) -> String {
        render(self.lines.iter(), None, &[], config, false)
    }

    /// Render escaped HTML with a theme and line emphasis.
    ///
    /// [`RenderHtmlStyle`] selects how token styles are emitted. A nonempty
    /// theme rainbow palette determines rainbow wrapping in class or inline mode.
    /// Variable mode ignores the supplied theme and uses `rainbow_colors` instead.
    pub fn render_html(&self, theme: &Theme, config: &RenderHtmlConfig) -> String {
        render(self.lines.iter(), Some(theme), &[], config, false)
    }

    /// Render inline styles for named themes with a selected default.
    ///
    /// The default theme uses literal CSS declarations. Other themes are embedded
    /// as `--comfit-<name>` properties on each element. Use CSS to activate them.
    /// The output contains no stylesheet or script. Each theme cycles through its own palette.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Html`] unless the style is [`RenderHtmlStyle::Inline`],
    /// the default theme exists, and names are nonempty ASCII letters, digits,
    /// underscores, or hyphens with no duplicate or colliding variable names.
    pub fn render_html_themes(
        &self,
        themes: &[(&str, &Theme)],
        default_theme: &str,
        config: &RenderHtmlConfig,
    ) -> Result<String, Error> {
        render_themes(self.lines.iter(), themes, default_theme, config)
    }
}

pub(crate) fn render_themes<'a, L: RenderLine + 'a>(
    lines: impl Iterator<Item = &'a L>,
    themes: &[(&str, &Theme)],
    default_theme: &str,
    config: &RenderHtmlConfig,
) -> Result<String, Error> {
    if config.style != RenderHtmlStyle::Inline {
        return Err(Error::Html("named themes require inline style".into()));
    }

    let mut names = std::collections::HashSet::new();

    for (name, _) in themes {
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err(Error::Html(format!("invalid theme name: {name:?}")));
        }

        for (_, suffix, _) in PROPERTIES {
            if !names.insert(format!("{name}{suffix}")) {
                return Err(Error::Html(format!(
                    "duplicate or colliding theme name: {name:?}"
                )));
            }
        }
    }

    let theme = themes
        .iter()
        .find(|(name, _)| *name == default_theme)
        .ok_or_else(|| Error::Html(format!("unknown default theme: {default_theme:?}")))?
        .1;
    let alternatives: Vec<_> = themes
        .iter()
        .copied()
        .filter(|(name, _)| *name != default_theme)
        .collect();
    Ok(render(lines, Some(theme), &alternatives, config, true))
}

pub(crate) fn render<'a, L: RenderLine + 'a>(
    lines: impl Iterator<Item = &'a L>,
    theme: Option<&Theme>,
    alternatives: &[(&str, &Theme)],
    config: &RenderHtmlConfig,
    complete: bool,
) -> String {
    let theme = theme.filter(|_| config.style != RenderHtmlStyle::Variables);
    let mut html = String::from("<");
    html.push_str(&config.pre_tag);
    let mut css = String::new();
    let mut scratch = String::new();

    if let Some(class) = &config.pre_class {
        attribute(&mut html, "class", class);
    }

    if config.style == RenderHtmlStyle::Variables {
        variable_style(&mut css, None, None, None);
        attribute(&mut html, "style", &css);
    } else if let Some(theme) = theme {
        inline_style(&mut css, &theme.default, None, complete, &mut scratch);

        for (name, alternative) in alternatives {
            inline_style(
                &mut css,
                &alternative.default,
                Some(name),
                true,
                &mut scratch,
            );
        }
        attribute(&mut html, "style", &css);
    }

    html.push_str("><code>");
    let mut prefix = String::new();
    let mut styles = HashMap::new();

    for (index, line) in lines.enumerate() {
        if index > 0 {
            html.push('\n');
        }

        html.push('<');
        html.push_str(&config.line_tag);
        html.push_str(" class=\"");
        escape(&mut html, &config.class_prefix);
        html.push_str("line");

        if line.highlighted() {
            html.push(' ');
            escape(&mut html, &config.class_prefix);
            html.push_str("line-highlight");
        }
        html.push('"');

        if config.style == RenderHtmlStyle::Variables && line.highlighted() {
            css.clear();
            variable_style(&mut css, None, Some("line-highlight"), None);
            attribute(&mut html, "style", &css);
        } else if let Some(theme) = theme
            && (line.highlighted() || complete)
        {
            css.clear();
            let empty = Style::default();
            let style = if line.highlighted() {
                &theme.line_highlight
            } else {
                &empty
            };
            inline_style(&mut css, style, None, complete, &mut scratch);

            for (name, alternative) in alternatives {
                let style = if line.highlighted() {
                    &alternative.line_highlight
                } else {
                    &empty
                };
                inline_style(&mut css, style, Some(name), true, &mut scratch);
            }
            attribute(&mut html, "style", &css);
        }

        html.push('>');

        for token in line.tokens() {
            html.push_str("<span");

            if config.style == RenderHtmlStyle::Inline
                && let Some(theme) = theme
            {
                let rainbow = if alternatives.is_empty() {
                    token.rainbow.and_then(|depth| {
                        (!theme.rainbow.is_empty()).then(|| depth % theme.rainbow.len())
                    })
                } else {
                    token.rainbow
                };
                let css = styles.entry((token.highlight, rainbow)).or_insert_with(|| {
                    let mut css = String::new();
                    let style = token_style(theme, token.highlight, token.rainbow);
                    inline_style(&mut css, &style, None, complete, &mut scratch);

                    for (name, alternative) in alternatives {
                        let style = token_style(alternative, token.highlight, token.rainbow);
                        inline_style(&mut css, &style, Some(name), true, &mut scratch);
                    }
                    css
                });
                attribute(&mut html, "style", css);
            } else {
                if config.style == RenderHtmlStyle::Variables {
                    css.clear();
                    let rainbow = token.rainbow.map(|depth| {
                        if config.rainbow_colors == 0 {
                            depth
                        } else {
                            depth % config.rainbow_colors
                        }
                    });
                    variable_style(&mut css, token.highlight, None, rainbow);
                    attribute(&mut html, "style", &css);
                }

                if token.highlight.is_some() || token.rainbow.is_some() {
                    html.push_str(" class=\"");
                }

                if let Some(capture) = token.highlight {
                    prefix.clear();

                    for (index, part) in capture.split('.').enumerate() {
                        if index > 0 {
                            html.push(' ');
                        }
                        if !prefix.is_empty() {
                            prefix.push('-');
                        }
                        prefix.push_str(part);
                        escape(&mut html, &config.class_prefix);
                        escape(&mut html, &prefix);
                    }
                }

                if let Some(depth) = token.rainbow {
                    let count = theme
                        .filter(|theme| !theme.rainbow.is_empty())
                        .map_or(config.rainbow_colors, |theme| theme.rainbow.len());
                    let color = if count == 0 { depth } else { depth % count };
                    if token.highlight.is_some() {
                        html.push(' ');
                    }
                    escape(&mut html, &config.class_prefix);
                    html.push_str("rainbow ");
                    escape(&mut html, &config.class_prefix);
                    write!(html, "rainbow-{color}").unwrap();
                }

                if token.highlight.is_some() || token.rainbow.is_some() {
                    html.push('"');
                }
            }

            html.push('>');
            escape(&mut html, token.text);
            html.push_str("</span>");
        }

        html.push_str("</");
        html.push_str(&config.line_tag);
        html.push('>');
    }

    html.push_str("</code></");
    html.push_str(&config.pre_tag);
    html.push('>');

    html
}

// CSS property, alternate-theme variable suffix, and the explicit reset value.
const PROPERTIES: [(&str, &str, &str); 9] = [
    ("color", "", "inherit"),
    ("background-color", "-bg", "transparent"),
    ("font-weight", "-font-weight", "normal"),
    ("font-style", "-font-style", "normal"),
    ("opacity", "-opacity", "1"),
    ("visibility", "-visibility", "visible"),
    ("text-decoration-line", "-text-decoration", "none"),
    ("text-decoration-style", "-text-decoration-style", "solid"),
    (
        "text-decoration-color",
        "-text-decoration-color",
        "currentColor",
    ),
];

fn token_style(theme: &Theme, capture: Option<&str>, rainbow: Option<usize>) -> Style {
    let mut style = capture.map_or_else(|| theme.default.clone(), |capture| theme.style(capture));

    // The default background belongs on <pre>; reversed colors still need it to swap.
    if !style.modifiers.contains(&Modifier::Reversed) {
        style.bg = capture.and_then(|capture| {
            let capture = capture.trim_start_matches('.');
            std::iter::once(capture.len())
                .chain(capture.rmatch_indices('.').map(|(end, _)| end))
                .find_map(|end| theme.styles.get(&capture[..end]).and_then(|style| style.bg))
        });
    }

    if let Some(depth) = rainbow
        && !theme.rainbow.is_empty()
    {
        style.fg = Some(theme.rainbow[depth % theme.rainbow.len()]);
    }

    style
}

fn inline_style(
    css: &mut String,
    style: &Style,
    name: Option<&str>,
    complete: bool,
    scratch: &mut String,
) {
    if !complete {
        style.write_css(css);
        return;
    }

    scratch.clear();
    style.write_css(scratch);
    let mut values = PROPERTIES.map(|(_, _, fallback)| fallback);

    for declaration in scratch.split(';') {
        if let Some((property, value)) = declaration.split_once(':')
            && let Some(index) = PROPERTIES.iter().position(|(key, _, _)| *key == property)
        {
            values[index] = value;
        }
    }

    for ((property, suffix, _), value) in PROPERTIES.iter().zip(values) {
        if let Some(name) = name {
            write!(css, "--comfit-{name}{suffix}:{value};").unwrap();
        } else {
            write!(css, "{property}:{value};").unwrap();
        }
    }
}

fn variable_style(
    css: &mut String,
    capture: Option<&str>,
    scope: Option<&str>,
    rainbow: Option<usize>,
) {
    let capture = capture
        .map(|capture| capture.trim_start_matches('.'))
        .filter(|capture| !capture.is_empty());

    for (property, suffix, fallback) in PROPERTIES {
        let suffix = if suffix.is_empty() {
            "fg"
        } else {
            &suffix[1..]
        };
        write!(css, "{property}:").unwrap();
        let rainbow = rainbow.filter(|_| property == "color");

        if let Some(depth) = rainbow {
            write!(css, "var(--comfit-rainbow-{depth},").unwrap();
        }

        if let Some(scope) = scope {
            write!(css, "var(--comfit-{scope}-{suffix},{fallback})").unwrap();
        } else {
            let mut parentheses = 0;

            if let Some(capture) = capture {
                for end in std::iter::once(capture.len())
                    .chain(capture.rmatch_indices('.').map(|(end, _)| end))
                {
                    css.push_str("var(--comfit-");

                    for character in capture[..end].chars() {
                        if character == '.' {
                            css.push('-');
                        } else if character.is_ascii_alphanumeric()
                            || matches!(character, '-' | '_')
                        {
                            css.push(character);
                        } else {
                            write!(css, "\\{:x} ", character as u32).unwrap();
                        }
                    }

                    write!(css, "-{suffix},").unwrap();
                    parentheses += 1;
                }
            }

            write!(css, "var(--comfit-{suffix},{fallback})").unwrap();

            for _ in 0..parentheses {
                css.push(')');
            }
        }

        if rainbow.is_some() {
            css.push(')');
        }
        css.push(';');
    }
}

fn attribute(html: &mut String, name: &str, value: &str) {
    html.push(' ');
    html.push_str(name);
    html.push_str("=\"");
    escape(html, value);
    html.push('"');
}

fn escape(html: &mut String, text: &str) {
    for character in text.chars() {
        match character {
            '&' => html.push_str("&amp;"),
            '<' => html.push_str("&lt;"),
            '>' => html.push_str("&gt;"),
            '"' => html.push_str("&quot;"),
            '\'' => html.push_str("&#39;"),
            // HTML parsing would normalize a literal CR independently of the next line's LF.
            '\r' => html.push_str("&#13;"),
            _ => html.push(character),
        }
    }
}
