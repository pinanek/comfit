//! Assign bracket depths from query captures and nesting scopes.

use std::collections::HashMap;

use crate::{backend::Match, query::Query};

use super::compose::Rainbow;

/// Collect source-ordered bracket depths for direct or explicitly included descendants.
///
/// `layer` records injection precedence. Depths remain independent of any color palette.
pub(crate) fn collect<Q>(query: &Query<Q>, matches: &[Match], layer: usize) -> Vec<Rainbow> {
    let mut scopes = HashMap::new();
    let mut brackets = HashMap::new();

    for matched in matches {
        for capture in &matched.captures {
            match query.capture_names[capture.index as usize].as_str() {
                "rainbow.scope" => {
                    scopes.insert(
                        capture.node.id,
                        (
                            &capture.node,
                            query.patterns[matched.pattern]
                                .property("rainbow.include-children")
                                .is_some(),
                        ),
                    );
                }
                "rainbow.bracket" => {
                    brackets.insert(
                        (capture.node.range.start, capture.node.range.end),
                        &capture.node,
                    );
                }
                _ => {}
            }
        }
    }

    let mut scopes: Vec<_> = scopes.into_values().collect();
    scopes.sort_by_key(|(node, _)| {
        (
            node.range.start,
            std::cmp::Reverse(node.range.end),
            node.depth,
        )
    });
    let mut brackets: Vec<_> = brackets.into_values().collect();
    brackets.sort_by_key(|node| (node.range.start, node.range.end));

    let mut pending = scopes.into_iter().peekable();
    let mut active: Vec<(&crate::backend::Node, bool)> = Vec::new();
    let mut positions = HashMap::new();
    let mut descendants = Vec::new();
    let mut result = Vec::new();

    for bracket in brackets {
        while pending
            .peek()
            .is_some_and(|(scope, _)| scope.range.start <= bracket.range.start)
        {
            let (scope, include_children) = pending.next().unwrap();

            while active
                .last()
                .is_some_and(|(node, _)| node.range.end <= scope.range.start)
            {
                let (node, include_children) = active.pop().unwrap();
                positions.remove(&node.id);

                if include_children {
                    descendants.pop();
                }
            }

            let depth = active.len();
            positions.insert(scope.id, depth);

            if include_children {
                descendants.push(depth);
            }

            active.push((scope, include_children));
        }

        while active
            .last()
            .is_some_and(|(scope, _)| scope.range.end <= bracket.range.start)
        {
            let (node, include_children) = active.pop().unwrap();
            positions.remove(&node.id);

            if include_children {
                descendants.pop();
            }
        }

        let direct = bracket
            .parent
            .and_then(|parent| positions.get(&parent))
            .copied();
        let indirect = descendants.last().copied();

        if let Some(depth) = direct.into_iter().chain(indirect).max() {
            result.push(Rainbow {
                range: bracket.range.clone(),
                depth,
                layer,
            });
        }
    }

    result
}
