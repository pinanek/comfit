//! Validate JSON themes for the shared renderer.

use std::collections::BTreeMap;

use comfit_core::{Color, Error, Modifier, Style, Theme, Underline, UnderlineStyle};
use serde::Deserialize;

/// A JSON theme converted to core colors, modifiers, and underline settings.
#[derive(Deserialize)]
#[serde(try_from = "ThemeInput")]
pub struct ThemeDefinition(
    /// Validated runtime theme, owned independently of the JSON input.
    pub Theme,
);

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
struct ThemeInput {
    name: String,
    default: StyleInput,
    styles: BTreeMap<String, StyleInput>,
    rainbow: Vec<ColorInput>,
    line_highlight: StyleInput,
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct StyleInput {
    fg: Option<ColorInput>,
    bg: Option<ColorInput>,
    modifiers: Vec<String>,
    underline: Option<UnderlineInput>,
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct UnderlineInput {
    color: Option<ColorInput>,
    style: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ColorInput {
    Indexed(u8),
    Text(String),
}

impl TryFrom<ThemeInput> for ThemeDefinition {
    type Error = Error;

    fn try_from(input: ThemeInput) -> Result<Self, Error> {
        Ok(Self(Theme {
            name: input.name,
            default: input.default.try_into()?,
            styles: input
                .styles
                .into_iter()
                .map(|(capture, style)| Ok((capture, style.try_into()?)))
                .collect::<Result<_, Error>>()?,
            rainbow: input
                .rainbow
                .into_iter()
                .map(Color::try_from)
                .collect::<Result<_, _>>()?,
            line_highlight: input.line_highlight.try_into()?,
        }))
    }
}

impl TryFrom<StyleInput> for Style {
    type Error = Error;

    fn try_from(input: StyleInput) -> Result<Self, Error> {
        Ok(Self {
            fg: input.fg.map(Color::try_from).transpose()?,
            bg: input.bg.map(Color::try_from).transpose()?,
            modifiers: input
                .modifiers
                .into_iter()
                .map(|modifier| {
                    Ok(match modifier.as_str() {
                        "bold" => Modifier::Bold,
                        "dim" => Modifier::Dim,
                        "italic" => Modifier::Italic,
                        "slow_blink" => Modifier::SlowBlink,
                        "rapid_blink" => Modifier::RapidBlink,
                        "reversed" => Modifier::Reversed,
                        "hidden" => Modifier::Hidden,
                        "crossed_out" => Modifier::CrossedOut,
                        _ => return Err(Error::Theme(format!("invalid modifier: {modifier}"))),
                    })
                })
                .collect::<Result<_, _>>()?,
            underline: input
                .underline
                .map(|underline| {
                    Ok(Underline {
                        color: underline.color.map(Color::try_from).transpose()?,
                        style: underline
                            .style
                            .map(|style| {
                                Ok(match style.as_str() {
                                    "line" => UnderlineStyle::Line,
                                    "curl" => UnderlineStyle::Curl,
                                    "dashed" => UnderlineStyle::Dashed,
                                    "dotted" => UnderlineStyle::Dotted,
                                    "double_line" => UnderlineStyle::DoubleLine,
                                    _ => {
                                        return Err(Error::Theme(format!(
                                            "invalid underline style: {style}"
                                        )));
                                    }
                                })
                            })
                            .transpose()?,
                    })
                })
                .transpose()?,
        })
    }
}

impl TryFrom<ColorInput> for Color {
    type Error = Error;

    fn try_from(input: ColorInput) -> Result<Self, Error> {
        match input {
            ColorInput::Indexed(index) => Ok(Color::Indexed(index)),
            ColorInput::Text(text) => text.parse(),
        }
    }
}
