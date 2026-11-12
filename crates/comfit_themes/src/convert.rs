//! Convert VS Code JSON/JSON5 themes to Comfit [`Theme`] values.
//!
//! Conversion is local and synchronous. This module does not load `include` files.
//! Resolve them with your own loader, then merge parent data before child data.
//! TextMate selectors are matched against representative Comfit capture stacks,
//! not used to tokenize source text.

use std::collections::{BTreeMap, HashSet};

use comfit_core::{Color, Error, Modifier, RGB8, Style, Theme, Underline, UnderlineStyle};
use serde::Deserialize;

mod scope;
mod toml;

pub use toml::to_toml;

/// A converted theme and its unmapped TextMate selectors.
pub struct Conversion {
    /// The converted theme, ready for rendering.
    pub theme: Theme,
    /// Unmapped selectors and their occurrence counts, sorted by selector.
    pub unmapped_selectors: BTreeMap<String, usize>,
}

/// Parse a self-contained VS Code JSON/JSON5 theme and convert it to [`Theme`].
///
/// Comments and trailing commas are supported. The supplied name overrides any
/// display name in the source. Use [`convert`] to also report unmapped selectors.
///
/// # Errors
///
/// Returns [`Error::Theme`] for invalid JSON, unresolved includes, unsupported
/// colors, or font modifiers used by the result.
///
/// # Examples
///
/// ```
/// let theme = comfit_themes::convert::from_vscode("Custom", r##"{
///     "colors": { "editor.foreground": "#cdd6f4" },
///     "tokenColors": [
///         { "scope": "entity.name.function", "settings": { "foreground": "#89b4fa" } }
///     ]
/// }"##)?;
/// assert!(theme.style("function").fg.is_some());
/// # Ok::<(), comfit_core::Error>(())
/// ```
pub fn from_vscode(name: &str, source: &str) -> Result<Theme, Error> {
    Ok(convert(name, &VscodeTheme::from_json(source)?)?.theme)
}

/// VS Code theme fields used by the converter.
#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct VscodeTheme {
    /// Optional parent theme resolved relative to the current theme.
    pub include: Option<String>,
    /// Editor defaults and bracket nesting colors.
    pub colors: VscodeColors,
    /// TextMate rules in source order.
    pub token_colors: Vec<TokenColor>,
}

/// Editor defaults and bracket colors used during conversion.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct VscodeColors {
    /// Foreground color string, if specified.
    #[serde(rename = "editor.foreground")]
    pub foreground: Option<String>,
    /// Background color string, if specified.
    #[serde(rename = "editor.background")]
    pub background: Option<String>,
    /// First bracket nesting color.
    #[serde(rename = "editorBracketHighlight.foreground1")]
    pub bracket_foreground_1: Option<String>,
    /// Second bracket nesting color.
    #[serde(rename = "editorBracketHighlight.foreground2")]
    pub bracket_foreground_2: Option<String>,
    /// Third bracket nesting color.
    #[serde(rename = "editorBracketHighlight.foreground3")]
    pub bracket_foreground_3: Option<String>,
    /// Fourth bracket nesting color.
    #[serde(rename = "editorBracketHighlight.foreground4")]
    pub bracket_foreground_4: Option<String>,
    /// Fifth bracket nesting color.
    #[serde(rename = "editorBracketHighlight.foreground5")]
    pub bracket_foreground_5: Option<String>,
    /// Sixth bracket nesting color.
    #[serde(rename = "editorBracketHighlight.foreground6")]
    pub bracket_foreground_6: Option<String>,
}

/// A TextMate selector rule and its style settings.
#[derive(Debug, Deserialize)]
pub struct TokenColor {
    /// Selectors. Unscoped rules are ignored, as in VS Code.
    pub scope: Option<Scopes>,
    /// Style properties applied by this rule.
    pub settings: Settings,
}

/// JSON selector forms accepted by VS Code themes.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Scopes {
    /// A single selector string, which may contain comma-separated alternatives.
    One(String),
    /// An array of selector strings.
    Many(Vec<String>),
}

/// Style properties read from a TextMate rule.
#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    /// Foreground color string, if specified.
    pub foreground: Option<String>,
    /// Background color string, if specified.
    pub background: Option<String>,
    /// Space-separated font modifiers. An empty string explicitly resets them.
    pub font_style: Option<String>,
}

impl VscodeTheme {
    /// Parse JSON/JSON5 without resolving includes or converting styles.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Theme`] when the source does not match the VS Code theme format.
    pub fn from_json(source: &str) -> Result<Self, Error> {
        json5::from_str(source).map_err(|error| Error::Theme(error.to_string()))
    }

    /// Append a child theme's rules and override each editor color it supplies.
    ///
    /// Merge parents first. Missing child colors retain their inherited values.
    /// Resolve and remove each `include` before merging the theme data.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Theme`] if either theme has an unresolved include, leaving
    /// this theme unchanged.
    pub fn merge(&mut self, child: Self) -> Result<(), Error> {
        if self.include.is_some() || child.include.is_some() {
            return Err(Error::Theme("resolve theme includes before merging".into()));
        }

        self.token_colors.extend(child.token_colors);

        for (inherited, child) in [
            (&mut self.colors.foreground, child.colors.foreground),
            (&mut self.colors.background, child.colors.background),
            (
                &mut self.colors.bracket_foreground_1,
                child.colors.bracket_foreground_1,
            ),
            (
                &mut self.colors.bracket_foreground_2,
                child.colors.bracket_foreground_2,
            ),
            (
                &mut self.colors.bracket_foreground_3,
                child.colors.bracket_foreground_3,
            ),
            (
                &mut self.colors.bracket_foreground_4,
                child.colors.bracket_foreground_4,
            ),
            (
                &mut self.colors.bracket_foreground_5,
                child.colors.bracket_foreground_5,
            ),
            (
                &mut self.colors.bracket_foreground_6,
                child.colors.bracket_foreground_6,
            ),
        ] {
            if child.is_some() {
                *inherited = child;
            }
        }

        Ok(())
    }
}

/// Convert VS Code theme data and report unmapped TextMate selectors.
///
/// The caller supplies the output name. Rules preserve source order and are
/// resolved with the same scope stacks used by the theme import tool.
/// Transparent `#RGBA` and `#RRGGBBAA` colors are composited over the token's
/// background, falling back to `editor.background`, because runtime themes store
/// opaque colors.
///
/// Scope mapping is approximate and does not evaluate semantic token rules.
/// Comfit cannot clear an underline inherited from a capture prefix.
///
/// # Errors
///
/// Returns [`Error::Theme`] for unresolved includes, invalid colors used by the
/// result, transparency without an opaque background, or unsupported font
/// modifiers. The converter does not download or open files.
pub fn convert(name: &str, parsed: &VscodeTheme) -> Result<Conversion, Error> {
    if parsed.include.is_some() {
        return Err(Error::Theme(
            "resolve theme includes before conversion".into(),
        ));
    }

    let background = parsed
        .colors
        .background
        .as_deref()
        .map(|value| color(value, None))
        .transpose()?;
    let mut theme = Theme {
        name: name.to_owned(),
        default: Style {
            fg: parsed
                .colors
                .foreground
                .as_deref()
                .map(|value| color(value, background))
                .transpose()?,
            bg: background,
            ..Style::default()
        },
        rainbow: [
            &parsed.colors.bracket_foreground_1,
            &parsed.colors.bracket_foreground_2,
            &parsed.colors.bracket_foreground_3,
            &parsed.colors.bracket_foreground_4,
            &parsed.colors.bracket_foreground_5,
            &parsed.colors.bracket_foreground_6,
        ]
        .into_iter()
        .filter_map(|color| color.as_ref())
        .map(|value| color(value, background))
        .collect::<Result<_, _>>()?,
        ..Theme::default()
    };
    let mut rules = Vec::new();

    for token in &parsed.token_colors {
        let scopes = match &token.scope {
            None => continue,
            Some(Scopes::One(scope)) => std::slice::from_ref(scope),
            Some(Scopes::Many(scopes)) => scopes.as_slice(),
        };

        for selector in scopes
            .iter()
            .flat_map(|scope| scope.split(','))
            .map(str::trim)
            .filter(|selector| !selector.is_empty())
        {
            rules.push((selector, &token.settings));
        }
    }

    let mut used = HashSet::new();

    for root in ["text", "source"] {
        for (selector, settings) in &rules {
            if *selector == root {
                apply(&mut theme.default, settings, background)?;
                used.insert(*selector);
            }
        }
    }

    for (capture, stacks) in scope::CAPTURES {
        let mut selected: Option<((bool, usize), Vec<_>)> = None;

        for stack in *stacks {
            let mut matching = Vec::new();
            let mut rank = (false, 0);

            for (selector, settings) in &rules {
                if let Some(specificity) = scope::specificity(selector, stack) {
                    used.insert(*selector);
                    rank = rank.max((
                        selector.split_whitespace().last() == stack.split_whitespace().last(),
                        selector.split_whitespace().count(),
                    ));
                    matching.push((specificity, *settings));
                }
            }

            // Prefer explicit leaf rules, then context; ties keep the canonical alias.
            if !matching.is_empty()
                && selected
                    .as_ref()
                    .is_none_or(|(previous_rank, _)| rank > *previous_rank)
            {
                selected = Some((rank, matching));
            }
        }

        let mut matching = selected.map(|(_, matching)| matching).unwrap_or_default();
        // Resolve each scope stack independently; Comfit capture prefixes may use unrelated scopes.
        matching.sort_by(|left, right| left.0.cmp(&right.0));
        let mut style = theme.default.clone();
        style.fg = Some(style.fg.unwrap_or(Color::Default));
        style.bg = Some(style.bg.unwrap_or(Color::Default));

        let mut resolved = Settings::default();
        for (_, settings) in matching {
            if settings.foreground.is_some() {
                resolved.foreground.clone_from(&settings.foreground);
            }
            if settings.background.is_some() {
                resolved.background.clone_from(&settings.background);
            }
            if settings.font_style.is_some() {
                resolved.font_style.clone_from(&settings.font_style);
            }
        }
        apply(&mut style, &resolved, background)?;

        theme.styles.insert((*capture).to_owned(), style);
    }

    let mut skipped = BTreeMap::new();

    for (selector, _) in rules {
        if !used.contains(selector) {
            *skipped.entry(selector.to_owned()).or_default() += 1;
        }
    }

    Ok(Conversion {
        theme,
        unmapped_selectors: skipped,
    })
}

fn color(value: &str, background: Option<Color>) -> Result<Color, Error> {
    if let Some(hex) = value
        .strip_prefix('#')
        .filter(|hex| hex.len() == 4 && hex.is_ascii())
    {
        let expanded: String = hex.chars().flat_map(|digit| [digit, digit]).collect();
        return color(&format!("#{expanded}"), background);
    }

    let Some(hex) = value
        .strip_prefix('#')
        .filter(|hex| hex.len() == 8 && hex.is_ascii())
    else {
        return value.parse();
    };
    let foreground: Color = value[..7].parse()?;
    let alpha = u8::from_str_radix(&hex[6..], 16)
        .map_err(|_| Error::Theme(format!("invalid color: {value:?}")))?;
    if alpha == 255 {
        return Ok(foreground);
    }
    let (Color::Rgb(foreground), Some(Color::Rgb(background))) = (foreground, background) else {
        return Err(Error::Theme(
            "transparent colors require an opaque background".into(),
        ));
    };
    let blend = |front: u8, back: u8| {
        ((u32::from(front) * u32::from(alpha) + u32::from(back) * (255 - u32::from(alpha)) + 127)
            / 255) as u8
    };
    Ok(Color::Rgb(RGB8::new(
        blend(foreground.r, background.r),
        blend(foreground.g, background.g),
        blend(foreground.b, background.b),
    )))
}

fn apply(style: &mut Style, settings: &Settings, background: Option<Color>) -> Result<(), Error> {
    if let Some(bg) = &settings.background {
        style.bg = Some(color(bg, background)?);
    }

    if let Some(fg) = &settings.foreground {
        style.fg = Some(color(fg, style.bg.or(background))?);
    }

    if let Some(font) = &settings.font_style {
        let mut modifiers = Vec::new();
        style.underline = None;

        for modifier in font.split_whitespace() {
            match modifier {
                "normal" | "regular" => {}
                "bold" => modifiers.push(Modifier::Bold),
                "italic" => modifiers.push(Modifier::Italic),
                "strikethrough" => modifiers.push(Modifier::CrossedOut),
                "underline" => {
                    style.underline = Some(Underline {
                        style: Some(UnderlineStyle::Line),
                        color: None,
                    })
                }
                _ => return Err(Error::Theme(format!("unsupported font style: {modifier}"))),
            }
        }
        style.modifiers = modifiers;
    }

    Ok(())
}
