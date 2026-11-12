//! Syntax highlighting and VS Code theme conversion for the command line.

mod ansi;
mod cli;
mod convert;

use std::{
    borrow::Cow,
    fs::File,
    io::{self, BufWriter, IsTerminal, Read, Write},
    path::Path,
    process::ExitCode,
};

use anyhow::{Context, Result, bail};
use clap::{CommandFactory, Parser};
use comfit::{HighlightConfig, Highlighter, RenderHtmlConfig, RenderHtmlStyle, Theme};

use cli::{Cli, Command, Highlight, HtmlStyle, Input};

fn main() -> ExitCode {
    let args = Cli::parse();
    let highlight = match &args.command {
        Command::Html(args) => Some(&args.highlight),
        Command::Ansi(args) => Some(&args.highlight),
        Command::Convert(_) => None,
    };

    if let Some(args) = highlight
        && !args.list_languages
        && !args.list_themes
        && args.language.is_none()
        && args.input.path().is_none()
    {
        Cli::command()
            .error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "--language is required when reading source from stdin",
            )
            .exit();
    }

    match run(args.command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error)
            if error
                .downcast_ref::<io::Error>()
                .is_some_and(|error| error.kind() == io::ErrorKind::BrokenPipe) =>
        {
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("comfit: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(command: Command) -> Result<()> {
    match command {
        Command::Convert(args) => convert::run(&args),
        Command::Html(args) => {
            if args.highlight.list()? {
                return Ok(());
            }
            let language = args.highlight.language()?;
            let theme = args.highlight.theme()?;
            let source = args.highlight.input.read()?;
            let mut highlighter = Highlighter::new()?;
            let highlighted =
                highlighter.highlight(&source, language, &args.highlight.config(&source))?;
            let html = highlighted.render_html(
                &theme,
                &RenderHtmlConfig {
                    pre_tag: args.pre_tag,
                    line_tag: args.line_tag,
                    pre_class: args.pre_class,
                    class_prefix: args.class_prefix,
                    rainbow_colors: args.rainbow_colors,
                    style: match args.style {
                        HtmlStyle::Class => RenderHtmlStyle::Class,
                        HtmlStyle::Inline => RenderHtmlStyle::Inline,
                        HtmlStyle::Variables => RenderHtmlStyle::Variables,
                    },
                },
            );
            args.highlight
                .input
                .write(|output| Ok(output.write_all(html.as_bytes())?))
        }
        Command::Ansi(args) => {
            if args.highlight.list()? {
                return Ok(());
            }
            let language = args.highlight.language()?;
            let theme = args.highlight.theme()?;
            let source = args.highlight.input.read()?;
            let mut highlighter = Highlighter::new()?;
            let highlighted =
                highlighter.highlight(&source, language, &args.highlight.config(&source))?;
            let terminal = args.highlight.input.stdout() && io::stdout().is_terminal();
            let layout = ansi::Layout::new(&args, &highlighted, terminal)?;
            args.highlight
                .input
                .write(|output| layout.write(output, &source, &highlighted, &theme))
        }
    }
}

impl Input {
    fn path(&self) -> Option<&Path> {
        self.file.as_deref().filter(|path| *path != Path::new("-"))
    }

    fn stdout(&self) -> bool {
        self.output
            .as_deref()
            .is_none_or(|path| path == Path::new("-"))
    }

    fn read(&self) -> Result<String> {
        if let Some(path) = self.path() {
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
        } else {
            let mut source = String::new();
            io::stdin()
                .lock()
                .read_to_string(&mut source)
                .context("reading stdin")?;
            Ok(source)
        }
    }

    fn write(&self, write: impl FnOnce(&mut dyn Write) -> Result<()>) -> Result<()> {
        if let Some(path) = self.output.as_deref().filter(|_| !self.stdout()) {
            let file =
                File::create(path).with_context(|| format!("creating {}", path.display()))?;
            let mut output = BufWriter::new(file);
            write(&mut output).with_context(|| format!("writing {}", path.display()))?;
            output
                .flush()
                .with_context(|| format!("flushing {}", path.display()))?;
        } else {
            let mut output = BufWriter::new(io::stdout().lock());
            write(&mut output)?;
            output.flush()?;
        }
        Ok(())
    }
}

impl Highlight {
    fn list(&self) -> Result<bool> {
        if !self.list_languages && !self.list_themes {
            return Ok(false);
        }

        self.input.write(|output| {
            if self.list_languages {
                for language in comfit::grammars::LANGUAGES {
                    write!(output, "{}\t{}", language.id, language.name)?;
                    for alias in language.aliases {
                        write!(output, "\t{alias}")?;
                    }
                    writeln!(output)?;
                }
            }
            if self.list_themes {
                for theme in comfit_themes::THEMES {
                    writeln!(output, "{}\t{}", theme.id, theme.name)?;
                }
            }
            Ok(())
        })?;
        Ok(true)
    }

    fn language(&self) -> Result<&str> {
        if let Some(language) = &self.language {
            if comfit::grammars::LANGUAGES.iter().any(|definition| {
                definition.id == language || definition.aliases.contains(&language.as_str())
            }) {
                return Ok(language);
            }
            bail!("unknown language {language:?}; use --list-languages");
        }

        let path = self.input.path();
        let filename = path
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str());
        let extension = path
            .and_then(|path| path.extension())
            .and_then(|extension| extension.to_str());
        if let Some((_, language)) =
            comfit::grammars::FILE_EXTENSIONS
                .iter()
                .find(|(candidate, _)| {
                    filename.is_some_and(|name| name.eq_ignore_ascii_case(candidate))
                        || extension
                            .is_some_and(|extension| extension.eq_ignore_ascii_case(candidate))
                })
        {
            return Ok(language);
        }

        bail!("cannot infer the source language; specify --language or use --list-languages")
    }

    fn theme(&self) -> Result<Cow<'static, Theme>> {
        if let Some(path) = &self.theme_file {
            let source = std::fs::read_to_string(path)
                .with_context(|| format!("reading {}", path.display()))?;
            return Ok(Cow::Owned(
                Theme::from_toml(&source)
                    .with_context(|| format!("loading theme {}", path.display()))?,
            ));
        }

        let id = self
            .theme
            .as_deref()
            .context("specify --theme or --theme-file")?;
        let definition = comfit_themes::THEMES
            .iter()
            .find(|theme| theme.id == id)
            .with_context(|| format!("unknown theme {id:?}; use --list-themes"))?;
        Ok(Cow::Borrowed((definition.theme)()))
    }

    fn config(&self, source: &str) -> HighlightConfig {
        HighlightConfig {
            rainbow: !self.no_rainbow,
            highlighted_lines: self
                .highlight_lines
                .as_ref()
                .map_or_else(Vec::new, |selected| {
                    (1..=source.split('\n').count())
                        .filter(|&line| selected.contains(line))
                        .collect()
                }),
        }
    }
}
