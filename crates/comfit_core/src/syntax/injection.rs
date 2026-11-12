//! Find embedded languages and merge their source byte ranges.

use std::{collections::BTreeMap, ops::Range};

use crate::{backend::Match, query::Query};

/// An embedded language to parse, with byte ranges in the original source.
pub(crate) struct Injection<'a> {
    /// Requested language ID or alias, borrowed from query metadata or source text.
    pub language: &'a str,
    /// Sorted byte ranges with overlapping or adjacent ranges merged.
    pub ranges: Vec<Range<usize>>,
}

/// Find injection languages, exclude named children, and limit content to the current layer.
///
/// Matches must already be validated against the query and source. Combined captures
/// are grouped by query pattern and language before merging their ranges.
pub(crate) fn collect<'a, Q>(
    query: &'a Query<Q>,
    matches: &[Match],
    source: &'a str,
    ranges: &[Range<usize>],
    language: &'a str,
    parent: Option<&'a str>,
) -> Vec<Injection<'a>> {
    let mut result = Vec::new();
    let mut combined: BTreeMap<(usize, &str), Vec<Range<usize>>> = BTreeMap::new();
    let mut included = Vec::new();

    for matched in matches {
        let pattern = &query.patterns[matched.pattern];
        let captured_language = matched
            .captures
            .iter()
            .find(|capture| query.capture_names[capture.index as usize] == "injection.language")
            .map(|capture| source[capture.node.range.clone()].trim());
        let name = captured_language
            .or_else(|| pattern.value("injection.language"))
            .or_else(|| pattern.property("injection.self").map(|_| language))
            .or_else(|| pattern.property("injection.parent").and(parent));
        let Some(name) = name else { continue };
        let mut contents = Vec::new();

        for capture in &matched.captures {
            if query.capture_names[capture.index as usize] != "injection.content" {
                continue;
            }

            let node = &capture.node;
            included.clear();
            if pattern.property("injection.include-children").is_some() {
                included.push(node.range.clone());
            } else {
                let mut start = node.range.start;

                for child in node.children.iter().filter(|child| child.named) {
                    if child.range.start >= node.range.end {
                        break;
                    }
                    if child.range.end <= start {
                        continue;
                    }
                    if child.range.start > start {
                        included.push(start..child.range.start);
                    }
                    start = child.range.end.min(node.range.end);
                }

                if start < node.range.end {
                    included.push(start..node.range.end);
                }
            }

            // Normalize once after all content captures have been clipped.
            for outer in ranges {
                for inner in &included {
                    let range = outer.start.max(inner.start)..outer.end.min(inner.end);

                    if !range.is_empty() {
                        contents.push(range);
                    }
                }
            }
        }

        if contents.is_empty() {
            continue;
        }

        if pattern.property("injection.combined").is_some() {
            combined
                .entry((matched.pattern, name))
                .or_default()
                .extend(contents);
        } else {
            result.push(Injection {
                language: name,
                ranges: normalize(contents),
            });
        }
    }

    result.extend(
        combined
            .into_iter()
            .map(|((_, language), ranges)| Injection {
                language,
                ranges: normalize(ranges),
            }),
    );

    result
}

fn normalize(mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    ranges.sort_by_key(|range| (range.start, range.end));
    ranges.dedup_by(|range, previous| {
        if range.start <= previous.end {
            previous.end = previous.end.max(range.end);
            true
        } else {
            false
        }
    });

    ranges
}
