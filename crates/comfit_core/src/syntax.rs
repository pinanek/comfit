//! Parse source and update root and embedded-language trees after edits.

use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Range,
};

use crate::{Error, backend::Backend, language::Registry};

mod injection;
mod stream;

pub(crate) use stream::walk;

/// A source edit covering all changes, using UTF-8 byte offsets and zero-based byte columns.
#[derive(Debug, Clone, Copy)]
pub struct Edit {
    pub start_byte: usize,
    pub old_end_byte: usize,
    pub new_end_byte: usize,
    pub start_position: (usize, usize),
    pub old_end_position: (usize, usize),
    pub new_end_position: (usize, usize),
}

impl Edit {
    pub(crate) fn between(old: &str, new: &str) -> Self {
        let mut start = old
            .bytes()
            .zip(new.bytes())
            .take_while(|(a, b)| a == b)
            .count();
        while !old.is_char_boundary(start) || !new.is_char_boundary(start) {
            start -= 1;
        }
        let mut suffix = old[start..]
            .bytes()
            .rev()
            .zip(new[start..].bytes().rev())
            .take_while(|(a, b)| a == b)
            .count();
        while !old.is_char_boundary(old.len() - suffix) || !new.is_char_boundary(new.len() - suffix)
        {
            suffix -= 1;
        }
        let position = |source: &str, end: usize| {
            let prefix = &source[..end];
            (
                prefix.bytes().filter(|byte| *byte == b'\n').count(),
                end - prefix.rfind('\n').map_or(0, |index| index + 1),
            )
        };
        Self {
            start_byte: start,
            old_end_byte: old.len() - suffix,
            new_end_byte: new.len() - suffix,
            start_position: position(old, start),
            old_end_position: position(old, old.len() - suffix),
            new_end_position: position(new, new.len() - suffix),
        }
    }
}

pub(crate) struct Layer<T> {
    pub language: String,
    pub ranges: Vec<Range<usize>>,
    pub parent: Option<usize>,
    pub depth: usize,
    pub tree: T,
}

/// Borrowed syntax data used by both document and one-shot highlighting.
pub(crate) struct LayerView<'a, T> {
    pub language: &'a str,
    pub ranges: &'a [Range<usize>],
    pub depth: usize,
    pub tree: &'a T,
}

impl<T> Layer<T> {
    pub fn view(&self) -> LayerView<'_, T> {
        LayerView {
            language: &self.language,
            ranges: &self.ranges,
            depth: self.depth,
            tree: &self.tree,
        }
    }

    pub(crate) fn adjust_ranges(&mut self, edit: &Edit) -> bool {
        for range in &mut self.ranges {
            if edit.old_end_byte < range.start {
                range.start = range.start - edit.old_end_byte + edit.new_end_byte;
                range.end = range.end - edit.old_end_byte + edit.new_end_byte;
            } else if edit.start_byte > range.start && edit.old_end_byte < range.end {
                range.end = range.end - edit.old_end_byte + edit.new_end_byte;
            } else if edit.start_byte <= range.end {
                // Boundary edits can change which language owns the content.
                return false;
            }
        }
        true
    }
}

/// Root and injection layers in parent-first order. The root is always at index zero.
pub(crate) struct Syntax<B: Backend> {
    pub layers: Vec<Layer<B::Tree>>,
}

struct ParseRequest<'a> {
    language: &'a str,
    ranges: Vec<Range<usize>>,
    parent: Option<usize>,
    previous_parent: Option<usize>,
    depth: usize,
}

impl<B: Backend> Syntax<B> {
    pub fn parse(
        backend: &mut B,
        registry: &Registry<B>,
        name: &str,
        source: &str,
        previous: Option<Self>,
        edit: Option<&Edit>,
    ) -> Result<Self, Error> {
        let language = registry.resolve(name)?;
        backend.prepare_source(source)?;
        let mut previous: HashMap<_, _> = previous
            .into_iter()
            .flat_map(|syntax| syntax.layers)
            .enumerate()
            .filter_map(|(index, mut layer)| {
                if layer.parent.is_none() {
                    layer.ranges = std::iter::once(0..source.len()).collect();
                } else if edit.is_some_and(|edit| !layer.adjust_ranges(edit)) {
                    return None;
                }
                Some((
                    (layer.parent, layer.language, layer.ranges),
                    (index, layer.tree),
                ))
            })
            .collect();
        let mut requests = VecDeque::from([ParseRequest {
            language,
            ranges: std::iter::once(0..source.len()).collect(),
            parent: None,
            previous_parent: None,
            depth: 0,
        }]);
        let mut seen = HashSet::new();
        let mut layers: Vec<Layer<B::Tree>> = Vec::new();

        while let Some(request) = requests.pop_front() {
            if request.depth > 32 {
                return Err(Error::InjectionDepth);
            }
            if !seen.insert((request.language, request.ranges.clone())) {
                continue;
            }
            let (raw, language) = registry.language(backend, request.language)?;
            let previous = if request.parent.is_none() {
                previous.remove(&(None, request.language.to_owned(), request.ranges.clone()))
            } else {
                request.previous_parent.and_then(|parent| {
                    previous.remove(&(
                        Some(parent),
                        request.language.to_owned(),
                        request.ranges.clone(),
                    ))
                })
            };
            let previous_index = previous.as_ref().map(|(index, _)| *index);
            let tree = backend.parse_prepared(
                raw,
                source,
                &request.ranges,
                previous.map(|(_, tree)| tree),
                edit,
            )?;
            let matches = language.injections.matches(backend, &tree, source)?;
            let index = layers.len();
            let parent_language = request
                .parent
                .map(|parent| layers[parent].language.as_str());
            for injection in injection::collect(
                &language.injections,
                &matches,
                source,
                &request.ranges,
                request.language,
                parent_language,
            ) {
                if let Some((target, _)) = registry.get(injection.language) {
                    requests.push_back(ParseRequest {
                        language: target,
                        ranges: injection.ranges,
                        parent: Some(index),
                        previous_parent: previous_index,
                        depth: request.depth + 1,
                    });
                }
            }
            layers.push(Layer {
                language: request.language.to_owned(),
                ranges: request.ranges,
                parent: request.parent,
                depth: request.depth,
                tree,
            });
        }
        Ok(Self { layers })
    }
}
