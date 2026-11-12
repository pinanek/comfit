//! Resolve overlapping captures into source lines and tokens.

use std::{collections::BTreeSet, ops::Range};

use crate::{HighlightConfig, Highlighted, Line, Token};

/// A styled node range with a rank for resolving overlapping captures.
pub(crate) struct Span<'a> {
    /// Node ID used to propagate local-definition styles within the current tree.
    pub node: usize,
    /// Half-open UTF-8 byte range in the original source.
    pub range: Range<usize>,
    /// Query capture name, or `None` to suppress an underlying style.
    pub highlight: Option<&'a str>,
    /// Layer, priority, reversed node width, and pattern index. Larger tuples win.
    pub rank: (usize, i32, std::cmp::Reverse<usize>, usize),
}

/// A bracket range with palette-independent depth and injection precedence.
pub(crate) struct Rainbow {
    /// Half-open UTF-8 byte range in the original source.
    pub range: Range<usize>,
    /// Zero-based nesting depth.
    pub depth: usize,
    /// Injection layer. Larger values win when bracket ranges overlap.
    pub layer: usize,
}

/// Build borrowed lines and tokens from capture boundaries, splitting at newlines.
///
/// # Panics
///
/// Panics if a span or bracket boundary does not describe a valid source slice.
pub(crate) fn lines<'a>(
    source: &'a str,
    spans: Vec<Span<'a>>,
    rainbows: Vec<Rainbow>,
    config: &HighlightConfig,
) -> Highlighted<'a> {
    let mut events = Vec::with_capacity(2 + 2 * spans.len() + 2 * rainbows.len());
    events.push((0, false, 0, usize::MAX));
    events.push((source.len(), false, 0, usize::MAX));

    for (index, span) in spans.iter().enumerate() {
        events.push((span.range.start, true, 0, index));
        events.push((span.range.end, false, 0, index));
    }

    for (index, rainbow) in rainbows.iter().enumerate() {
        events.push((rainbow.range.start, true, 1, index));
        events.push((rainbow.range.end, false, 1, index));
    }
    events.sort_unstable();

    let mut result = Highlighted {
        lines: vec![Line {
            number: 1,
            highlighted: false,
            tokens: Vec::new(),
        }],
    };

    let mut styles = BTreeSet::<(_, usize)>::new();
    let mut colors = BTreeSet::<(_, usize)>::new();
    let mut position = 0;

    for (offset, start, kind, index) in events {
        if offset > position {
            let highlight = styles.last().and_then(|&(_, index)| spans[index].highlight);
            let rainbow = colors.last().map(|&(_, index)| rainbows[index].depth);
            push(&mut result, &source[position..offset], highlight, rainbow);
            position = offset;
        }

        if index == usize::MAX {
            continue;
        }

        if kind == 0 {
            let key = (spans[index].rank, index);

            if start {
                styles.insert(key);
            } else {
                styles.remove(&key);
            }
        } else {
            let rainbow = &rainbows[index];
            let key = (
                (rainbow.layer, std::cmp::Reverse(rainbow.range.len())),
                index,
            );

            if start {
                colors.insert(key);
            } else {
                colors.remove(&key);
            }
        }
    }

    let mut selected = config.highlighted_lines.clone();
    selected.sort_unstable();
    let mut selected = selected.into_iter().peekable();

    for line in &mut result.lines {
        while selected.peek().is_some_and(|number| *number < line.number) {
            selected.next();
        }
        line.highlighted = selected.peek() == Some(&line.number);
    }

    result
}

fn push<'a>(
    result: &mut Highlighted<'a>,
    text: &'a str,
    highlight: Option<&'a str>,
    rainbow: Option<usize>,
) {
    for (index, text) in text.split('\n').enumerate() {
        if index > 0 {
            let number = result.lines.len() + 1;
            result.lines.push(Line {
                number,
                highlighted: false,
                tokens: Vec::new(),
            });
        }

        if !text.is_empty() {
            let tokens = &mut result.lines.last_mut().unwrap().tokens;
            tokens.push(Token {
                text,
                highlight,
                rainbow,
            });
        }
    }
}
