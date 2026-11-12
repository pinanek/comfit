//! Generate scoped theme stylesheets for Comfit's class-based HTML output.

use std::{fmt::Write, fs, path::Path};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use comfit_core::{Modifier, Style, Theme};

#[derive(Parser)]
#[command(about = "Generate CSS from Comfit TOML themes")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate one stylesheet per theme in npm/css/dist.
    Build {
        /// Match the HTML renderer's classPrefix option.
        #[arg(long, default_value = "")]
        class_prefix: String,
    },
}

fn main() -> Result<()> {
    let Args { command } = Args::parse();
    let root = comfit_internal::workspace_root();

    match command {
        Command::Build { class_prefix } => build(root, &class_prefix),
    }
}

fn build(root: &Path, class_prefix: &str) -> Result<()> {
    let mut paths = fs::read_dir(root.join("crates/comfit_themes/data"))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    paths.retain(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "toml"));
    paths.sort();

    let staging = root.join("target/comfit-css");

    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    fs::create_dir_all(&staging)?;

    for path in &paths {
        let id = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .with_context(|| format!("invalid theme filename: {}", path.display()))?;
        let theme = Theme::from_toml(&fs::read_to_string(path)?)
            .with_context(|| format!("loading {}", path.display()))?;
        fs::write(
            staging.join(format!("{id}.css")),
            stylesheet(id, &theme, class_prefix),
        )?;
    }

    let output = root.join("npm/css/dist");
    fs::create_dir_all(output.parent().context("missing CSS output parent")?)?;

    if output.exists() {
        fs::remove_dir_all(&output)?;
    }
    fs::rename(staging, &output)?;
    println!(
        "Generated {} theme stylesheets in {}",
        paths.len(),
        output.display()
    );

    Ok(())
}

/// Render theme rules scoped to the `pre.comfit-<id>` selector.
fn stylesheet(id: &str, theme: &Theme, class_prefix: &str) -> String {
    let scope = format!("pre.{}", identifier(&format!("comfit-{id}")));
    let line = identifier(&format!("{class_prefix}line"));
    let mut css =
        String::from("/* Generated theme styles; regenerate with cargo css build. */\n\n");
    writeln!(css, "{scope} {{ {} }}", theme.default.to_css()).unwrap();
    writeln!(
        css,
        "{scope} :where(code > .{line} > span) {{ --comfit-rainbow: initial; {} }}",
        token_style(&theme.default)
    )
    .unwrap();

    let mut captures: Vec<_> = theme.styles.keys().collect();
    captures.sort_by_key(|capture| capture.bytes().filter(|&byte| byte == b'.').count());

    for capture in captures {
        let class = identifier(&format!("{class_prefix}{}", capture.replace('.', "-")));
        writeln!(
            css,
            "{scope} .{class} {{ {} }}",
            token_style(&theme.style(capture))
        )
        .unwrap();
    }

    let line_highlight = identifier(&format!("{class_prefix}line-highlight"));
    writeln!(
        css,
        "{scope} .{line_highlight} {{ {} }}",
        theme.line_highlight.to_css()
    )
    .unwrap();

    for (index, color) in theme.rainbow.iter().enumerate() {
        let class = identifier(&format!("{class_prefix}rainbow-{index}"));
        writeln!(
            css,
            "{scope} .{class} {{ --comfit-rainbow: {}; }}",
            color.to_css()
        )
        .unwrap();
    }

    css
}

fn token_style(style: &Style) -> String {
    // Captures share one element, so absent modifiers must reset earlier rules.
    let mut css = String::from(
        "color:inherit;background-color:transparent;font-weight:normal;font-style:normal;opacity:1;visibility:visible;text-decoration:none;",
    );
    css.push_str(&style.to_css());

    // Core applies rainbow to the foreground before swapping reversed colors.
    let (property, fallback) = if style.modifiers.contains(&Modifier::Reversed) {
        ("background-color", "transparent")
    } else {
        ("color", "inherit")
    };
    let color = style
        .fg
        .map_or_else(|| fallback.to_owned(), |color| color.to_css());
    write!(css, "{property}:var(--comfit-rainbow,{color});").unwrap();

    css
}

fn identifier(value: &str) -> String {
    let mut escaped = String::new();

    for (index, character) in value.chars().enumerate() {
        if character.is_ascii_alphabetic()
            || character == '_'
            || (index > 0 && (character.is_ascii_digit() || character == '-'))
        {
            escaped.push(character);
        } else {
            write!(escaped, "\\{:x} ", character as u32).unwrap();
        }
    }

    escaped
}
