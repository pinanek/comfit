//! Capture styles, terminal colors, and optional TOML theme loading.

use std::{collections::BTreeMap, fmt::Write};

use rgb::RGB8;

/// Default styles, capture styles, bracket colors, and line emphasis.
#[derive(Debug, Clone, Default)]
pub struct Theme {
    /// Display name from the theme source.
    pub name: String,
    /// Base style inherited by tokens.
    pub default: Style,
    /// Capture styles keyed by dotted query capture names.
    pub styles: BTreeMap<String, Style>,
    /// Bracket colors cycled by nesting depth. An empty palette disables bracket coloring.
    pub rainbow: Vec<Color>,
    /// Style applied to selected lines by a renderer.
    pub line_highlight: Style,
}

/// Optional colors, font modifiers, and underline settings.
#[derive(Debug, Clone, Default)]
pub struct Style {
    /// Foreground color, if explicitly set.
    pub fg: Option<Color>,
    /// Background color, if explicitly set.
    pub bg: Option<Color>,
    /// Font and visibility modifiers. An empty list resets inherited modifiers.
    pub modifiers: Vec<Modifier>,
    /// Underline settings, if explicitly set.
    pub underline: Option<Underline>,
}

/// Truecolor, indexed terminal color, or the output default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// A 24-bit RGB color.
    Rgb(RGB8),
    /// An index in the 256-color terminal palette.
    Indexed(u8),
    /// The terminal default, represented as `inherit` in CSS.
    Default,
}

/// Optional underline color and shape.
#[derive(Debug, Clone, Default)]
pub struct Underline {
    /// Underline color. Uses the output default if omitted.
    pub color: Option<Color>,
    /// Underline shape. Uses a straight line if omitted.
    pub style: Option<UnderlineStyle>,
}

/// Underline shapes supported by the renderers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlineStyle {
    /// A single straight line.
    Line,
    /// A wavy underline.
    Curl,
    /// A dashed underline.
    Dashed,
    /// A dotted underline.
    Dotted,
    /// Two straight lines.
    DoubleLine,
}

/// Font, visibility, and terminal display modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    /// Bold text.
    Bold,
    /// Dimmed text.
    Dim,
    /// Italic text.
    Italic,
    /// Slow terminal blinking. CSS uses blink decoration.
    SlowBlink,
    /// Rapid terminal blinking. CSS uses blink decoration.
    RapidBlink,
    /// Swap foreground and background colors.
    Reversed,
    /// Hide text without removing its space.
    Hidden,
    /// Strike through text.
    CrossedOut,
}

impl Theme {
    /// Combine the default style with styles matching the capture name and its prefixes.
    ///
    /// More specific styles override colors and underlines when those values are set.
    /// Each matching style replaces the modifier list, including an empty list.
    /// Leading dots are ignored when building prefixes. Empty prefixes still match
    /// the empty style key. Only the final modifier list is cloned.
    ///
    /// # Examples
    ///
    /// ```
    /// use comfit_core::{Color, Style, Theme};
    ///
    /// let mut theme = Theme::default();
    /// theme.styles.insert("function".into(), Style {
    ///     fg: Some(Color::Indexed(4)),
    ///     ..Style::default()
    /// });
    ///
    /// // A more specific capture inherits matching prefix styles.
    /// assert_eq!(theme.style("function.call").fg, Some(Color::Indexed(4)));
    /// ```
    pub fn style(&self, capture: &str) -> Style {
        let mut result = Style {
            fg: self.default.fg,
            bg: self.default.bg,
            underline: self.default.underline.clone(),
            modifiers: Vec::new(),
        };
        let mut modifiers = &self.default.modifiers;
        let normalized = capture.trim_start_matches('.');
        let mut end = 0;

        for part in capture.split('.') {
            if end > 0 {
                end += 1;
            }
            end += part.len();

            if let Some(style) = self.styles.get(&normalized[..end]) {
                if style.fg.is_some() {
                    result.fg = style.fg;
                }

                if style.bg.is_some() {
                    result.bg = style.bg;
                }

                modifiers = &style.modifiers;

                if style.underline.is_some() {
                    result.underline.clone_from(&style.underline);
                }
            }
        }

        result.modifiers = modifiers.clone();
        result
    }

    /// Load default, capture, rainbow, and line-highlight styles from TOML.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Theme`] for invalid TOML or unsupported color, modifier,
    /// underline, or table values.
    #[cfg(feature = "theme-toml")]
    pub fn from_toml(source: &str) -> Result<Self, crate::Error> {
        let value: toml::Value =
            toml::from_str(source).map_err(|error| crate::Error::Theme(error.to_string()))?;
        let mut theme = Self {
            name: value
                .get("name")
                .and_then(toml::Value::as_str)
                .unwrap_or_default()
                .to_owned(),
            ..Self::default()
        };

        if let Some(default) = value.get("default") {
            theme.default = parse_style(default)?;
        }

        if let Some(styles) = value.get("styles") {
            let styles = styles
                .as_table()
                .ok_or_else(|| crate::Error::Theme("styles must be a table".into()))?;

            for (name, style) in styles {
                theme.styles.insert(name.clone(), parse_style(style)?);
            }
        }

        if let Some(colors) = value.get("rainbow") {
            let colors = colors
                .as_array()
                .ok_or_else(|| crate::Error::Theme("rainbow must be an array".into()))?;

            for color in colors {
                theme.rainbow.push(parse_color(color)?);
            }
        }

        if let Some(style) = value.get("line_highlight") {
            theme.line_highlight = parse_style(style)?;
        }

        Ok(theme)
    }
}

impl Color {
    /// Convert a color to CSS, using the standard terminal palette for indexed colors.
    pub fn to_css(self) -> String {
        let mut css = String::new();
        self.write_css(&mut css);
        css
    }

    fn write_css(self, css: &mut String) {
        let color = match self {
            Self::Default => {
                css.push_str("inherit");
                return;
            }
            Self::Rgb(color) => color,
            Self::Indexed(index) if index < 16 => {
                const COLORS: [(u8, u8, u8); 16] = [
                    (0, 0, 0),
                    (128, 0, 0),
                    (0, 128, 0),
                    (128, 128, 0),
                    (0, 0, 128),
                    (128, 0, 128),
                    (0, 128, 128),
                    (192, 192, 192),
                    (128, 128, 128),
                    (255, 0, 0),
                    (0, 255, 0),
                    (255, 255, 0),
                    (0, 0, 255),
                    (255, 0, 255),
                    (0, 255, 255),
                    (255, 255, 255),
                ];
                COLORS[index as usize].into()
            }

            Self::Indexed(index) if index >= 232 => {
                let shade = 8 + (index - 232) * 10;
                RGB8::new(shade, shade, shade)
            }

            Self::Indexed(index) => {
                let index = index - 16;
                let level = |value: u8| if value == 0 { 0 } else { 55 + value * 40 };
                RGB8::new(level(index / 36), level(index / 6 % 6), level(index % 6))
            }
        };

        write!(css, "#{:02x}{:02x}{:02x}", color.r, color.g, color.b).unwrap();
    }
}

impl Style {
    /// Render CSS declarations for explicitly set style properties.
    ///
    /// Reversed colors are swapped before rendering. Terminal blink speeds both map
    /// to CSS blink decoration. Browser support determines the visible result.
    pub fn to_css(&self) -> String {
        let mut css = String::new();
        self.write_css(&mut css);
        css
    }

    /// Append declarations for explicitly set properties without clearing the buffer.
    pub(crate) fn write_css(&self, css: &mut String) {
        let (foreground, background) = if self.modifiers.contains(&Modifier::Reversed) {
            (self.bg, self.fg)
        } else {
            (self.fg, self.bg)
        };

        if let Some(color) = foreground {
            css.push_str("color:");
            color.write_css(css);
            css.push(';');
        }

        if let Some(color) = background {
            css.push_str("background-color:");
            color.write_css(css);
            css.push(';');
        }

        for modifier in &self.modifiers {
            css.push_str(match modifier {
                Modifier::Bold => "font-weight:bold;",
                Modifier::Dim => "opacity:0.5;",
                Modifier::Italic => "font-style:italic;",
                Modifier::Hidden => "visibility:hidden;",
                Modifier::CrossedOut => "text-decoration-line:line-through;",
                Modifier::SlowBlink | Modifier::RapidBlink => "text-decoration-line:blink;",
                Modifier::Reversed => "",
            });
        }

        if let Some(underline) = &self.underline {
            let crossed = self.modifiers.contains(&Modifier::CrossedOut);
            css.push_str(if crossed {
                "text-decoration-line:underline line-through;"
            } else {
                "text-decoration-line:underline;"
            });
            let style = match underline.style.unwrap_or(UnderlineStyle::Line) {
                UnderlineStyle::Line => "solid",
                UnderlineStyle::Curl => "wavy",
                UnderlineStyle::Dashed => "dashed",
                UnderlineStyle::Dotted => "dotted",
                UnderlineStyle::DoubleLine => "double",
            };

            css.push_str("text-decoration-style:");
            css.push_str(style);
            css.push(';');

            if let Some(color) = underline.color {
                css.push_str("text-decoration-color:");
                color.write_css(css);
                css.push(';');
            }
        }
    }
}

impl std::str::FromStr for Color {
    type Err = crate::Error;

    /// Parse a color from `#RGB`, `#RRGGBB`, or `default`.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::Theme`] for unsupported formats or invalid hex digits.
    fn from_str(color: &str) -> Result<Self, Self::Err> {
        match color {
            "default" => Ok(Color::Default),
            color
                if color.len() == 4
                    && color.starts_with('#')
                    && color[1..].bytes().all(|byte| byte.is_ascii_hexdigit()) =>
            {
                let value = u16::from_str_radix(&color[1..], 16).unwrap();

                Ok(Color::Rgb(RGB8::new(
                    ((value >> 8) as u8) * 17,
                    ((value >> 4 & 15) as u8) * 17,
                    ((value & 15) as u8) * 17,
                )))
            }
            color
                if color.len() == 7
                    && color.starts_with('#')
                    && color[1..].bytes().all(|byte| byte.is_ascii_hexdigit()) =>
            {
                let value = u32::from_str_radix(&color[1..], 16).unwrap();

                Ok(Color::Rgb(RGB8::new(
                    (value >> 16) as u8,
                    (value >> 8) as u8,
                    value as u8,
                )))
            }
            _ => Err(crate::Error::Theme(format!("invalid color: {color:?}"))),
        }
    }
}

#[cfg(feature = "theme-toml")]
fn parse_color(value: &toml::Value) -> Result<Color, crate::Error> {
    if let Some(index) = value
        .as_integer()
        .and_then(|index| u8::try_from(index).ok())
    {
        return Ok(Color::Indexed(index));
    }

    value
        .as_str()
        .and_then(|color| color.parse().ok())
        .ok_or_else(|| crate::Error::Theme(format!("invalid color: {value}")))
}

#[cfg(feature = "theme-toml")]
fn parse_style(value: &toml::Value) -> Result<Style, crate::Error> {
    let value = value
        .as_table()
        .ok_or_else(|| crate::Error::Theme("style must be a table".into()))?;

    let mut style = Style::default();

    if let Some(color) = value.get("fg") {
        style.fg = Some(parse_color(color)?);
    }

    if let Some(color) = value.get("bg") {
        style.bg = Some(parse_color(color)?);
    }

    if let Some(modifiers) = value.get("modifiers") {
        let modifiers = modifiers
            .as_array()
            .ok_or_else(|| crate::Error::Theme("modifiers must be an array".into()))?;

        for modifier in modifiers {
            style.modifiers.push(match modifier.as_str() {
                Some("bold") => Modifier::Bold,
                Some("dim") => Modifier::Dim,
                Some("italic") => Modifier::Italic,
                Some("slow_blink") => Modifier::SlowBlink,
                Some("rapid_blink") => Modifier::RapidBlink,
                Some("reversed") => Modifier::Reversed,
                Some("hidden") => Modifier::Hidden,
                Some("crossed_out") => Modifier::CrossedOut,
                _ => return Err(crate::Error::Theme(format!("invalid modifier: {modifier}"))),
            });
        }
    }

    if let Some(value) = value.get("underline") {
        let value = value
            .as_table()
            .ok_or_else(|| crate::Error::Theme("underline must be a table".into()))?;
        let color = value.get("color").map(parse_color).transpose()?;
        let underline = match value
            .get("style")
            .and_then(toml::Value::as_str)
            .unwrap_or("line")
        {
            "line" => UnderlineStyle::Line,
            "curl" => UnderlineStyle::Curl,
            "dashed" => UnderlineStyle::Dashed,
            "dotted" => UnderlineStyle::Dotted,
            "double_line" => UnderlineStyle::DoubleLine,
            other => {
                return Err(crate::Error::Theme(format!(
                    "invalid underline style: {other}"
                )));
            }
        };
        style.underline = Some(Underline {
            color,
            style: Some(underline),
        });
    }

    Ok(style)
}
