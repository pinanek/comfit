//! Traverse one-shot syntax layers without retaining parsed trees.

use std::{
    collections::{HashSet, VecDeque},
    ops::Range,
};

use crate::{Error, backend::Backend, language::Registry};

use super::{LayerView, injection};

struct Request<'a> {
    language: &'a str,
    parent_language: Option<&'a str>,
    ranges: Vec<Range<usize>>,
    depth: usize,
}

/// Visit each parsed layer, releasing its tree before parsing the next one.
pub(crate) fn walk<B: Backend>(
    backend: &mut B,
    registry: &Registry<B>,
    name: &str,
    source: &str,
    mut visit: impl FnMut(&mut B, LayerView<'_, B::Tree>) -> Result<(), Error>,
) -> Result<(), Error> {
    let mut requests = VecDeque::from([Request {
        language: registry.resolve(name)?,
        parent_language: None,
        ranges: std::iter::once(0..source.len()).collect(),
        depth: 0,
    }]);
    backend.prepare_source(source)?;
    let mut seen = HashSet::new();

    while let Some(request) = requests.pop_front() {
        if request.depth > 32 {
            return Err(Error::InjectionDepth);
        }
        if !seen.insert((request.language, request.ranges.clone())) {
            continue;
        }
        let (raw, language) = registry.language(backend, request.language)?;
        let tree = backend.parse_prepared(raw, source, &request.ranges, None, None)?;
        visit(
            backend,
            LayerView {
                language: request.language,
                ranges: &request.ranges,
                depth: request.depth,
                tree: &tree,
            },
        )?;

        let matches = language.injections.matches(backend, &tree, source)?;
        for injection in injection::collect(
            &language.injections,
            &matches,
            source,
            &request.ranges,
            request.language,
            request.parent_language,
        ) {
            if let Some((target, _)) = registry.get(injection.language) {
                requests.push_back(Request {
                    language: target,
                    parent_language: Some(request.language),
                    ranges: injection.ranges,
                    depth: request.depth + 1,
                });
            }
        }
    }
    Ok(())
}
