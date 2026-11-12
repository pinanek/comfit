//! Render ANSI-styled text and escape source control characters.

use std::fmt::Write;

use crate::{Color, Highlighted, Modifier, Style, Theme, Token, UnderlineStyle};

use super::RenderLine;

impl Highlighted<'_> {
    /// Render ANSI-styled source using a theme.
    ///
    /// Reset styles after each token and escape source controls other than tabs and
    /// newlines. This renderer does not add terminal-width padding or line-number gutters.
    pub fn render_ansi(&self, theme: &Theme) -> String {
        render(self.lines.iter(), theme)
    }
}

pub(crate) fn render<'a, L: RenderLine + 'a>(
    lines: impl Iterator<Item = &'a L>,
    theme: &Theme,
) -> String {
    let mut ansi = String::new();

    for (index, line) in lines.enumerate() {
        if index > 0 {
            ansi.push('\n');
        }

        for token in line.tokens() {
            token.write_ansi(&mut ansi, theme, line.highlighted());
        }
    }

    ansi
}

impl Token<'_> {
    /// Append a styled token to a buffer, escaping source control characters.
    /// `highlighted` applies the theme's selected-line style before rainbow colors.
    pub fn write_ansi(&self, ansi: &mut String, theme: &Theme, highlighted: bool) {
        if self.text.is_empty() {
            return;
        }

        let mut style = self
            .highlight
            .map_or_else(|| theme.default.clone(), |capture| theme.style(capture));

        if highlighted {
            style.fg = theme.line_highlight.fg.or(style.fg);
            style.bg = theme.line_highlight.bg.or(style.bg);

            for modifier in &theme.line_highlight.modifiers {
                if !style.modifiers.contains(modifier) {
                    style.modifiers.push(*modifier);
                }
            }

            if theme.line_highlight.underline.is_some() {
                style.underline.clone_from(&theme.line_highlight.underline);
            }
        }

        if let Some(depth) = self.rainbow
            && !theme.rainbow.is_empty()
        {
            style.fg = Some(theme.rainbow[depth % theme.rainbow.len()]);
        }

        write_style(ansi, &style);

        for character in self.text.chars() {
            if character.is_control() && !matches!(character, '\n' | '\t') {
                ansi.extend(character.escape_default());
            } else {
                ansi.push(character);
            }
        }

        ansi.push_str("\x1b[0m");
    }
}

fn write_style(ansi: &mut String, style: &Style) {
    ansi.push_str("\x1b[0");

    if let Some(color) = style.fg {
        write_color(ansi, color, 38);
    }

    if let Some(color) = style.bg {
        write_color(ansi, color, 48);
    }

    for modifier in &style.modifiers {
        ansi.push_str(match modifier {
            Modifier::Bold => ";1",
            Modifier::Dim => ";2",
            Modifier::Italic => ";3",
            Modifier::SlowBlink => ";5",
            Modifier::RapidBlink => ";6",
            Modifier::Reversed => ";7",
            Modifier::Hidden => ";8",
            Modifier::CrossedOut => ";9",
        });
    }

    if let Some(underline) = &style.underline {
        ansi.push_str(match underline.style.unwrap_or(UnderlineStyle::Line) {
            UnderlineStyle::Line => ";4",
            UnderlineStyle::Curl => ";4:3",
            UnderlineStyle::Dashed => ";4:5",
            UnderlineStyle::Dotted => ";4:4",
            UnderlineStyle::DoubleLine => ";4:2",
        });

        if let Some(color) = underline.color {
            write_color(ansi, color, 58);
        }
    }

    ansi.push('m');
}

fn write_color(ansi: &mut String, color: Color, parameter: u8) {
    match color {
        Color::Rgb(color) => {
            write!(ansi, ";{parameter};2;{};{};{}", color.r, color.g, color.b).unwrap();
        }
        Color::Indexed(index) => {
            write!(ansi, ";{parameter};5;{index}").unwrap();
        }
        Color::Default => {
            write!(ansi, ";{}", parameter + 1).unwrap();
        }
    }
}
