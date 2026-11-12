//! Evaluate parsed syntax layers and compose their highlights.

use crate::{
    Error,
    backend::Backend,
    language::Registry,
    syntax::{self, LayerView, Syntax},
};

mod compose;
mod locals;
mod rainbow;

pub use crate::output::{
    HighlightConfig, HighlightResult, Highlighted, Line, OwnedLine, OwnedToken, Token,
};

use compose::Span;
use locals::Bindings;

struct Highlights<'a> {
    spans: Vec<Span<'a>>,
    rainbows: Vec<compose::Rainbow>,
}

impl<'a> Highlights<'a> {
    fn new() -> Self {
        Self {
            spans: Vec::new(),
            rainbows: Vec::new(),
        }
    }

    fn collect<B: Backend>(
        &mut self,
        backend: &mut B,
        registry: &'a Registry<B>,
        source: &'a str,
        layer: LayerView<'_, B::Tree>,
        config: &HighlightConfig,
    ) -> Result<(), Error> {
        let (_, language) = registry.language(backend, layer.language)?;
        let tree = layer.tree;
        let ranges = layer.ranges;
        let depth = layer.depth;
        let local_matches = language.locals.matches(backend, tree, source)?;
        let bindings = Bindings::resolve(&language.locals, &local_matches, source);
        drop(local_matches);
        let highlights = language.highlights.matches(backend, tree, source)?;
        let layer_start = self.spans.len();

        for matched in highlights {
            let pattern = &language.highlights.patterns[matched.pattern];
            let priority = pattern.priority;

            for capture in matched.captures {
                let name = language.highlights.capture_names[capture.index as usize].as_str();

                if name.starts_with('_') || matches!(name, "spell" | "nospell") {
                    continue;
                }

                if pattern
                    .local
                    .is_some_and(|required| required != bindings.is_local(capture.node.id))
                {
                    continue;
                }

                let highlight = (name != "none").then_some(name);

                for included in ranges {
                    let range = included.start.max(capture.node.range.start)
                        ..included.end.min(capture.node.range.end);

                    if range.is_empty() {
                        continue;
                    }

                    self.spans.push(Span {
                        node: capture.node.id,
                        rank: (
                            depth,
                            priority,
                            std::cmp::Reverse(capture.node.range.len()),
                            matched.pattern,
                        ),
                        range,
                        highlight,
                    });
                }
            }
        }

        bindings.apply(&mut self.spans[layer_start..]);
        drop(bindings);

        if config.rainbow {
            let matches = language.rainbows.matches(backend, tree, source)?;

            for color in rainbow::collect(&language.rainbows, &matches, depth) {
                for included in ranges {
                    let range =
                        included.start.max(color.range.start)..included.end.min(color.range.end);

                    if range.is_empty() {
                        continue;
                    }

                    self.rainbows.push(compose::Rainbow {
                        range,
                        depth: color.depth,
                        layer: color.layer,
                    });
                }
            }
        }
        Ok(())
    }

    fn finish(self, source: &'a str, config: &HighlightConfig) -> Highlighted<'a> {
        compose::lines(source, self.spans, self.rainbows, config)
    }
}

pub(crate) fn run<'a, B: Backend>(
    backend: &mut B,
    registry: &'a Registry<B>,
    source: &'a str,
    syntax: &Syntax<B>,
    config: &HighlightConfig,
) -> Result<Highlighted<'a>, Error> {
    let mut highlights = Highlights::new();
    for layer in &syntax.layers {
        highlights.collect(backend, registry, source, layer.view(), config)?;
    }
    Ok(highlights.finish(source, config))
}

pub(crate) fn source<'a, B: Backend>(
    backend: &mut B,
    registry: &'a Registry<B>,
    source: &'a str,
    name: &str,
    config: &HighlightConfig,
) -> Result<Highlighted<'a>, Error> {
    let mut highlights = Highlights::new();
    syntax::walk(backend, registry, name, source, |backend, layer| {
        highlights.collect(backend, registry, source, layer, config)
    })?;
    Ok(highlights.finish(source, config))
}
