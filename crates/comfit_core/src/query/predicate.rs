//! Compile custom query predicates and adjust capture byte ranges.

use crate::{Error, backend::Match};

use super::Argument;

/// A compiled custom predicate or capture-offset directive.
pub enum Predicate {
    /// Require captured text to contain at least one literal.
    Contains {
        /// Capture index to inspect or adjust.
        capture: u32,
        /// Literal substrings to search for.
        values: Vec<String>,
    },
    /// Test parent or ancestor node kinds, optionally negating the result.
    Ancestor {
        /// Capture index to inspect or adjust.
        capture: u32,
        /// Node kinds that satisfy the structural test.
        kinds: Vec<String>,
        /// Inspect only the immediate parent when true.
        parent: bool,
        /// Accept when no inspected node matches a kind.
        negative: bool,
    },
    /// Adjust capture boundaries by row and byte-column offsets.
    Offset {
        /// Capture index to inspect or adjust.
        capture: u32,
        /// Start row/column and end row/column adjustments, in that order.
        offsets: [isize; 4],
    },
}

impl Predicate {
    /// Compile a custom-predicate operator and literal arguments.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Query`] for unsupported operators or malformed arguments.
    pub fn compile(operator: &str, arguments: Vec<Argument>) -> Result<Self, Error> {
        let mut arguments = arguments.into_iter();
        let Some(Argument::Capture(capture)) = arguments.next() else {
            return Err(Error::Query(format!("#{operator} requires a capture")));
        };

        let values = arguments
            .map(|argument| match argument {
                Argument::String(value) => Ok(value),
                _ => Err(Error::Query(format!(
                    "#{operator} requires literal arguments"
                ))),
            })
            .collect::<Result<Vec<_>, _>>()?;

        match operator {
            "contains?" if !values.is_empty() => Ok(Self::Contains { capture, values }),
            "has-parent?" | "not-has-parent?" | "has-ancestor?" | "not-has-ancestor?"
                if !values.is_empty() =>
            {
                Ok(Self::Ancestor {
                    capture,
                    kinds: values,
                    parent: operator.ends_with("parent?"),
                    negative: operator.starts_with("not-"),
                })
            }
            "offset!" if values.len() == 4 => {
                let mut offsets = [0; 4];

                for (offset, value) in offsets.iter_mut().zip(values) {
                    *offset = value
                        .parse()
                        .map_err(|_| Error::Query("invalid #offset! argument".into()))?;
                }

                Ok(Self::Offset { capture, offsets })
            }
            _ => Err(Error::Query(format!(
                "unsupported or malformed predicate: #{operator}"
            ))),
        }
    }

    /// Test all matching captures before applying offsets.
    ///
    /// A predicate with no matching captures is accepted. Offset directives do not
    /// filter matches. [`Self::apply`] adjusts their ranges.
    pub fn accepts(&self, matched: &Match, source: &str) -> bool {
        let capture_index = match self {
            Self::Contains { capture, .. }
            | Self::Ancestor { capture, .. }
            | Self::Offset { capture, .. } => *capture,
        };
        matched
            .captures
            .iter()
            .filter(|capture| capture.index == capture_index)
            .all(|capture| {
                let text = source.get(capture.node.range.clone()).unwrap_or_default();

                match self {
                    Self::Contains { values, .. } => {
                        values.iter().any(|value| text.contains(value))
                    }

                    Self::Ancestor {
                        kinds,
                        parent,
                        negative,
                        ..
                    } => {
                        let count = if *parent {
                            1
                        } else {
                            capture.node.ancestors.len()
                        };

                        let found = capture
                            .node
                            .ancestors
                            .iter()
                            .take(count)
                            .any(|kind| kinds.contains(kind));
                        found != *negative
                    }

                    Self::Offset { .. } => true,
                }
            })
    }

    /// Apply offset directives to matching captures. Other predicates do nothing.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRange`] when an offset leaves the source or splits
    /// a UTF-8 character.
    pub fn apply(&self, matched: &mut Match, source: &str) -> Result<(), Error> {
        let Self::Offset { capture, offsets } = self else {
            return Ok(());
        };

        for node in matched
            .captures
            .iter_mut()
            .filter(|item| item.index == *capture)
            .map(|item| &mut item.node)
        {
            let start = offset(source, node.range.start, offsets[0], offsets[1]);
            let end = offset(source, node.range.end, offsets[2], offsets[3]);
            let range = start
                .zip(end)
                .map(|(start, end)| start..end)
                .filter(|range| source.get(range.clone()).is_some())
                .ok_or_else(|| Error::InvalidRange(node.range.clone()))?;
            node.range = range;
        }

        Ok(())
    }
}

fn offset(source: &str, byte: usize, rows: isize, columns: isize) -> Option<usize> {
    let prefix = source.get(..byte)?;
    let mut start = prefix.rfind('\n').map_or(0, |newline| newline + 1);
    let column = (byte - start).checked_add_signed(columns)?;

    for _ in 0..rows.unsigned_abs() {
        start = if rows > 0 {
            start + source[start..].find('\n')? + 1
        } else {
            let end = start.checked_sub(1)?;
            source[..end].rfind('\n').map_or(0, |newline| newline + 1)
        };
    }

    let length = source[start..].find('\n').unwrap_or(source.len() - start);

    if column > length {
        return None;
    }

    let byte = start.checked_add(column)?;

    source.is_char_boundary(byte).then_some(byte)
}
