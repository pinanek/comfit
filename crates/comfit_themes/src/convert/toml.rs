//! Serialize themes as Comfit TOML.

use comfit_core::{Color, Modifier, Style, Theme, UnderlineStyle};
use toml_edit::{Array, DocumentMut, InlineTable, Item, Table, Value, value};

/// Serialize a theme as Comfit TOML, including explicit modifier resets.
///
/// The output can be loaded with `comfit_core::Theme::from_toml` when its
/// `theme-toml` feature is enabled. Underlines without a shape use a straight line.
pub fn to_toml(theme: &Theme) -> String {
    let mut document = DocumentMut::new();
    document["name"] = value(&theme.name);
    document["default"] = Item::Value(Value::InlineTable(render_style(&theme.default)));

    if !theme.rainbow.is_empty() {
        let rainbow: Array = theme.rainbow.iter().copied().map(render_color).collect();
        document["rainbow"] = value(rainbow);
    }

    let mut styles = Table::new();

    for (scope, style) in &theme.styles {
        styles.insert(scope, Item::Value(Value::InlineTable(render_style(style))));
    }

    document["styles"] = Item::Table(styles);

    let line = &theme.line_highlight;
    if line.fg.is_some()
        || line.bg.is_some()
        || !line.modifiers.is_empty()
        || line.underline.is_some()
    {
        document["line_highlight"] = Item::Value(Value::InlineTable(render_style(line)));
    }

    document.to_string()
}

fn render_style(style: &Style) -> InlineTable {
    let mut table = InlineTable::new();

    if let Some(fg) = style.fg {
        table.insert("fg", render_color(fg));
    }

    if let Some(bg) = style.bg {
        table.insert("bg", render_color(bg));
    }

    let modifiers: Array = style
        .modifiers
        .iter()
        .map(|modifier| match modifier {
            Modifier::Bold => "bold",
            Modifier::Dim => "dim",
            Modifier::Italic => "italic",
            Modifier::SlowBlink => "slow_blink",
            Modifier::RapidBlink => "rapid_blink",
            Modifier::Reversed => "reversed",
            Modifier::Hidden => "hidden",
            Modifier::CrossedOut => "crossed_out",
        })
        .collect();
    table.insert("modifiers", Value::Array(modifiers));

    if let Some(underline) = &style.underline {
        let mut underline_table = InlineTable::new();
        let shape = match underline.style.unwrap_or(UnderlineStyle::Line) {
            UnderlineStyle::Line => "line",
            UnderlineStyle::Curl => "curl",
            UnderlineStyle::Dashed => "dashed",
            UnderlineStyle::Dotted => "dotted",
            UnderlineStyle::DoubleLine => "double_line",
        };
        underline_table.insert("style", Value::from(shape));

        if let Some(color) = underline.color {
            underline_table.insert("color", render_color(color));
        }
        table.insert("underline", Value::InlineTable(underline_table));
    }

    table.fmt();

    table
}

fn render_color(color: Color) -> Value {
    match color {
        Color::Indexed(index) => Value::from(i64::from(index)),
        Color::Default => Value::from("default"),
        Color::Rgb(_) => Value::from(color.to_css()),
    }
}
