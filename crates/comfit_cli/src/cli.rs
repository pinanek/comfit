//! Command-line arguments and validated line selections.

use std::{num::NonZeroUsize, ops::RangeInclusive, path::PathBuf, str::FromStr};

use clap::{Args, Parser, Subcommand, ValueEnum};

/// Top-level CLI command selection.
#[derive(Parser)]
#[command(
    name = "comfit",
    version,
    about = "Highlight source code and convert VS Code themes"
)]
pub(crate) struct Cli {
    /// Command to execute after argument validation.
    #[command(subcommand)]
    pub command: Command,
}

/// Highlighting and theme-conversion commands.
#[derive(Subcommand)]
pub(crate) enum Command {
    /// Render an escaped HTML fragment.
    Html(Html),
    /// Render source for a terminal, with optional line numbers and wrapping.
    Ansi(Ansi),
    /// Convert a VS Code JSON/JSON5 theme to Comfit TOML.
    Convert(Convert),
}

/// Source and destination paths shared by all commands.
#[derive(Args)]
pub(crate) struct Input {
    /// Input file. Omit or use "-" to read stdin.
    pub file: Option<PathBuf>,
    /// Write to a file instead of stdout.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

/// Language, theme, and source-line options shared by render commands.
#[derive(Args)]
pub(crate) struct Highlight {
    /// Shared source and destination paths.
    #[command(flatten)]
    pub input: Input,
    /// Language ID or alias. Inferred from the file extension if omitted.
    #[arg(short, long)]
    pub language: Option<String>,
    /// Bundled theme ID. Required unless --theme-file or a listing option is supplied.
    #[arg(
        short,
        long,
        conflicts_with = "theme_file",
        required_unless_present_any = ["theme_file", "list_languages", "list_themes"]
    )]
    pub theme: Option<String>,
    /// Load a custom Comfit TOML theme.
    #[arg(long)]
    pub theme_file: Option<PathBuf>,
    /// Emphasize one-based source lines, e.g. "1,4-8".
    #[arg(long, value_name = "SPEC")]
    pub highlight_lines: Option<LineSelection>,
    /// Disable rainbow bracket colors.
    #[arg(long)]
    pub no_rainbow: bool,
    /// List bundled language IDs and aliases without reading input.
    #[arg(long)]
    pub list_languages: bool,
    /// List bundled theme IDs without reading input.
    #[arg(long)]
    pub list_themes: bool,
}

/// HTML element classes and token-style options.
#[derive(Args)]
pub(crate) struct Html {
    /// Shared language, theme, and highlighting options.
    #[command(flatten)]
    pub highlight: Highlight,
    /// Represent token styles with classes, inline values, or CSS variables.
    #[arg(long, value_enum, default_value = "inline")]
    pub style: HtmlStyle,
    /// Prefix generated CSS classes.
    #[arg(long, default_value = "")]
    pub class_prefix: String,
    /// Outer tag name.
    #[arg(long, default_value = "pre")]
    pub pre_tag: String,
    /// Line tag name.
    #[arg(long, default_value = "span")]
    pub line_tag: String,
    /// Class attribute for the outer element.
    #[arg(long)]
    pub pre_class: Option<String>,
    /// Rainbow wrapping count. Zero preserves depth. Class/inline themes use their palette.
    #[arg(long, default_value_t = 0)]
    pub rainbow_colors: usize,
}

/// Ways to style tokens in HTML output.
#[derive(Clone, Copy, ValueEnum)]
pub(crate) enum HtmlStyle {
    /// Emit capture classes for a caller-provided stylesheet.
    Class,
    /// Embed literal theme styles in token elements.
    Inline,
    /// Reference caller-provided CSS variables instead of theme values.
    Variables,
}

/// Terminal width, wrapping, color policy, and source-line selection.
#[derive(Args)]
pub(crate) struct Ansi {
    /// Shared language, theme, and highlighting options.
    #[command(flatten)]
    pub highlight: Highlight,
    /// Show original source line numbers.
    #[arg(short = 'n', long)]
    pub line_numbers: bool,
    /// Total display columns, including the gutter. Enables character wrapping.
    #[arg(long, value_name = "COLUMNS")]
    pub width: Option<NonZeroUsize>,
    /// Wrapping mode [default: never, or character when --width is supplied].
    #[arg(long, value_enum)]
    pub wrap: Option<Wrap>,
    /// Expand tabs to tab stops measured from the start of each display row.
    #[arg(long, default_value = "4", value_name = "COLUMNS")]
    pub tab_width: NonZeroUsize,
    /// Auto disables styling for files, redirected stdout, NO_COLOR, or TERM=dumb.
    #[arg(long, value_enum, default_value = "auto")]
    pub color: ColorWhen,
    /// Display an inclusive source-line range while highlighting the whole source.
    #[arg(long, value_name = "START:END")]
    pub line_range: Option<LineRange>,
}

/// How to wrap source lines in terminal output.
#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum Wrap {
    /// Do not insert wrap newlines.
    Never,
    /// Wrap at Unicode grapheme boundaries.
    Character,
    /// Prefer word boundaries and split longer words at grapheme boundaries.
    Word,
}

/// Policy for emitting ANSI style codes.
#[derive(Clone, Copy, ValueEnum)]
pub(crate) enum ColorWhen {
    /// Style terminal output unless a nonempty NO_COLOR or TERM=dumb disables it.
    Auto,
    /// Emit styles even for redirected output or a disabled terminal environment.
    Always,
    /// Emit plain text without ANSI style codes or background padding.
    Never,
}

/// VS Code theme input, output name, and unmapped-selector reporting.
#[derive(Args)]
pub(crate) struct Convert {
    /// Shared source and destination paths.
    #[command(flatten)]
    pub input: Input,
    /// Override the source theme's display name (otherwise use its name or file stem).
    #[arg(short, long)]
    pub name: Option<String>,
    /// Report unmapped selectors and occurrence counts to stderr.
    #[arg(long)]
    pub report_unmapped: bool,
}

/// Validated one-based line numbers and inclusive ranges for emphasis.
#[derive(Clone)]
pub(crate) struct LineSelection(Vec<RangeInclusive<usize>>);

impl LineSelection {
    /// Return whether the selection contains the supplied source line number.
    pub fn contains(&self, line: usize) -> bool {
        self.0.iter().any(|range| range.contains(&line))
    }
}

impl FromStr for LineSelection {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value
            .split(',')
            .map(|part| {
                let (start, end) = part.split_once('-').unwrap_or((part, part));
                line_range(start, end)
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }
}

/// An inclusive source-line range with positive, ordered bounds.
#[derive(Clone)]
pub(crate) struct LineRange(pub RangeInclusive<usize>);

impl FromStr for LineRange {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (start, end) = value.split_once(':').ok_or("expected START:END")?;
        line_range(start, end).map(Self)
    }
}

fn line_range(start: &str, end: &str) -> Result<RangeInclusive<usize>, String> {
    let parse = |value: &str| {
        value
            .trim()
            .parse::<NonZeroUsize>()
            .map(NonZeroUsize::get)
            .map_err(|_| format!("invalid one-based line number: {value:?}"))
    };
    let start = parse(start)?;
    let end = parse(end)?;

    if start > end {
        return Err("line range start must not exceed its end".into());
    }

    Ok(start..=end)
}
