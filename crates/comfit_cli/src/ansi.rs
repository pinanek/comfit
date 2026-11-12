//! Add Unicode-aware wrapping and line numbers to the shared ANSI renderer.

use std::{
    env,
    fmt::Write as _,
    io::Write,
    ops::{Range, RangeInclusive},
};

use anyhow::{Result, ensure};
use comfit::{Highlighted, Modifier, Theme, Token};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::cli::{Ansi, ColorWhen, Wrap};

/// Terminal layout, color settings, and selected source lines.
pub(crate) struct Layout {
    width: usize,
    columns: Option<usize>,
    gutter: usize,
    tab_width: usize,
    wrap: Wrap,
    color: bool,
    lines: Option<RangeInclusive<usize>>,
}

impl Layout {
    /// Resolve layout options, capping display width to the terminal when available.
    ///
    /// # Errors
    ///
    /// Returns an error if wrapping leaves no content columns beyond the gutter.
    pub fn new(args: &Ansi, highlighted: &Highlighted<'_>, terminal: bool) -> Result<Self> {
        let wrap = args.wrap.unwrap_or(if args.width.is_some() {
            Wrap::Character
        } else {
            Wrap::Never
        });
        let line_count = highlighted.lines.len()
            - usize::from(
                highlighted.lines.len() > 1
                    && highlighted
                        .lines
                        .last()
                        .is_some_and(|line| line.tokens.is_empty()),
            );
        let gutter = if args.line_numbers {
            let last = args
                .line_range
                .as_ref()
                .map_or(line_count, |range| (*range.0.end()).min(line_count));
            last.max(1).ilog10() as usize + 4
        } else {
            0
        };
        let terminal_width = if terminal {
            terminal_size::terminal_size()
                .map(|(width, _)| usize::from(width.0))
                .filter(|&width| width > 0)
        } else {
            None
        };
        let width = args
            .width
            .map(|width| width.get())
            .or(terminal_width)
            .unwrap_or(80);
        // Padding beyond the terminal edge would add an unintended display row.
        let width = terminal_width.map_or(width, |terminal| width.min(terminal));
        let columns = if wrap == Wrap::Never {
            None
        } else {
            ensure!(
                width > gutter,
                "display width {width} must exceed the {gutter}-column line-number gutter"
            );
            Some(width - gutter)
        };
        let color = match args.color {
            ColorWhen::Always => true,
            ColorWhen::Never => false,
            ColorWhen::Auto => {
                terminal
                    && !env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty())
                    && env::var_os("TERM").is_none_or(|value| value != "dumb")
            }
        };

        Ok(Self {
            width,
            columns,
            gutter,
            tab_width: args.tab_width.get(),
            wrap,
            color,
            lines: args.line_range.as_ref().map(|range| range.0.clone()),
        })
    }

    /// Write selected source lines with theme styles and optional terminal layout.
    ///
    /// `source` must be the text used to produce `highlighted`.
    ///
    /// # Errors
    ///
    /// Returns an error for output failures or graphemes wider than the content area.
    pub fn write(
        &self,
        output: &mut dyn Write,
        source: &str,
        highlighted: &Highlighted<'_>,
        theme: &Theme,
    ) -> Result<()> {
        let mut row = Row::default();

        for (index, (text, line)) in source.split('\n').zip(&highlighted.lines).enumerate() {
            // The highlighter's final empty line represents the trailing separator,
            // not another numbered source line.
            if text.is_empty() && index > 0 && index + 1 == highlighted.lines.len() {
                break;
            }
            if self
                .lines
                .as_ref()
                .is_some_and(|range| !range.contains(&line.number))
            {
                continue;
            }

            row.number = line.number;
            row.highlighted = line.highlighted;
            row.continuation = false;
            row.column = if self.gutter == 0 {
                0
            } else {
                (self.gutter - 1) % self.width + 1
            };
            let mut tokens = line.tokens.iter();
            let mut token = tokens.next();
            let mut token_end = token.map_or(0, |token| token.text.len());
            let mut offset = 0;

            let mut words = text.split_word_bounds();
            let mut whole = std::iter::once(text);
            let segments: &mut dyn Iterator<Item = &str> = if self.wrap == Wrap::Word {
                &mut words
            } else {
                &mut whole
            };

            for word in segments {
                if self.wrap == Wrap::Word
                    && !word.chars().all(char::is_whitespace)
                    && let Some(columns) = self.columns
                    && row.width > 0
                    && display_width(word, row.width, self.tab_width) > columns - row.width
                {
                    row.newline(output, self, theme)?;
                }

                for grapheme in word.graphemes(true) {
                    while offset >= token_end && token.is_some() {
                        token = tokens.next();
                        token_end += token.map_or(0, |token| token.text.len());
                    }
                    let Some(style) = token else {
                        break;
                    };

                    if grapheme == "\t" {
                        if self.columns == Some(row.width) {
                            row.newline(output, self, theme)?;
                        }
                        let spaces = self.tab_width - row.width % self.tab_width;
                        for _ in 0..spaces {
                            row.push(" ", 1, style, output, self, theme)?;
                        }
                    } else if grapheme.chars().any(char::is_control) {
                        for character in grapheme.chars() {
                            if character.is_control() {
                                for escaped in character.escape_default() {
                                    let mut bytes = [0; 4];
                                    row.push(
                                        escaped.encode_utf8(&mut bytes),
                                        1,
                                        style,
                                        output,
                                        self,
                                        theme,
                                    )?;
                                }
                            } else {
                                let mut bytes = [0; 4];
                                let text = character.encode_utf8(&mut bytes);
                                row.push(text, text.width(), style, output, self, theme)?;
                            }
                        }
                    } else {
                        row.push(grapheme, grapheme.width(), style, output, self, theme)?;
                    }
                    offset += grapheme.len();
                }
            }

            row.write(output, self, theme)?;
            if index + 1 < highlighted.lines.len() {
                output.write_all(b"\n")?;
            }
        }

        Ok(())
    }
}

fn display_width(text: &str, column: usize, tab_width: usize) -> usize {
    let mut end = column;
    for grapheme in text.graphemes(true) {
        let width = if grapheme == "\t" {
            tab_width - end % tab_width
        } else if grapheme.chars().any(char::is_control) {
            grapheme
                .chars()
                .map(|character| {
                    if character.is_control() {
                        character.escape_default().count()
                    } else {
                        character.width().unwrap_or(0)
                    }
                })
                .sum()
        } else {
            grapheme.width()
        };
        end = end.saturating_add(width);
    }
    end - column
}

struct Span<'a> {
    range: Range<usize>,
    highlight: Option<&'a str>,
    rainbow: Option<usize>,
}

#[derive(Default)]
struct Row<'a> {
    text: String,
    gutter: String,
    ansi: String,
    spans: Vec<Span<'a>>,
    width: usize,
    column: usize,
    number: usize,
    highlighted: bool,
    continuation: bool,
}

impl<'a> Row<'a> {
    fn push(
        &mut self,
        text: &str,
        width: usize,
        style: &Token<'a>,
        output: &mut dyn Write,
        layout: &Layout,
        theme: &Theme,
    ) -> Result<()> {
        if let Some(columns) = layout.columns {
            ensure!(
                width <= columns,
                "a source grapheme needs {width} columns but only {columns} are available; increase --width"
            );
            if self.width + width > columns {
                self.newline(output, layout, theme)?;
            }
        }

        let start = self.text.len();
        self.text.push_str(text);
        if layout.color {
            if let Some(last) = self.spans.last_mut()
                && last.highlight == style.highlight
                && last.rainbow == style.rainbow
            {
                last.range.end = self.text.len();
            } else {
                self.spans.push(Span {
                    range: start..self.text.len(),
                    highlight: style.highlight,
                    rainbow: style.rainbow,
                });
            }
        }
        if width > 0 {
            // Wide graphemes soft-wrap before the last column if they cannot fit.
            if width > layout.width.saturating_sub(self.column) {
                self.column = 0;
            }
            self.column += width;
        }
        self.width += width;
        Ok(())
    }

    fn newline(&mut self, output: &mut dyn Write, layout: &Layout, theme: &Theme) -> Result<()> {
        self.write(output, layout, theme)?;
        output.write_all(b"\n")?;
        self.continuation = true;
        self.column = layout.gutter;
        Ok(())
    }

    fn has_background(&self, theme: &Theme) -> bool {
        theme.default.bg.is_some()
            || theme.default.modifiers.contains(&Modifier::Reversed)
            || self.highlighted
                && (theme.line_highlight.bg.is_some()
                    || theme.line_highlight.modifiers.contains(&Modifier::Reversed))
    }

    fn write(&mut self, output: &mut dyn Write, layout: &Layout, theme: &Theme) -> Result<()> {
        self.gutter.clear();
        if layout.gutter > 0 {
            if self.continuation {
                self.gutter.extend(std::iter::repeat_n(' ', layout.gutter));
            } else {
                write!(
                    &mut self.gutter,
                    "{:>width$} | ",
                    self.number,
                    width = layout.gutter - 3
                )
                .unwrap();
            }
        }

        if layout.color {
            let padding = if self.has_background(theme) {
                layout.width.saturating_sub(self.column)
            } else {
                0
            };
            let padding_start = self.text.len();
            self.text.extend(std::iter::repeat_n(' ', padding));
            self.ansi.clear();
            Token {
                text: &self.gutter,
                highlight: None,
                rainbow: None,
            }
            .write_ansi(&mut self.ansi, theme, self.highlighted);

            for span in &self.spans {
                Token {
                    text: &self.text[span.range.clone()],
                    highlight: span.highlight,
                    rainbow: span.rainbow,
                }
                .write_ansi(&mut self.ansi, theme, self.highlighted);
            }

            Token {
                text: &self.text[padding_start..],
                highlight: None,
                rainbow: None,
            }
            .write_ansi(&mut self.ansi, theme, self.highlighted);
            output.write_all(self.ansi.as_bytes())?;
        } else {
            output.write_all(self.gutter.as_bytes())?;
            output.write_all(self.text.as_bytes())?;
        }

        self.text.clear();
        self.spans.clear();
        self.width = 0;
        self.column = 0;
        Ok(())
    }
}
