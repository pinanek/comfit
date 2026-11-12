//! Registry data, validation, and utilities for workspace tools.

use std::path::Path;

pub mod command;
pub mod grammar;
pub mod theme;

/// Return the repository root regardless of the current working directory.
pub fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}

/// Identify upstream license and notice filenames without case sensitivity.
pub fn is_license_filename(name: &str) -> bool {
    ["LICENSE", "COPYING", "COPYRIGHT", "NOTICE"]
        .into_iter()
        .any(|prefix| {
            name.get(..prefix.len())
                .is_some_and(|start| start.eq_ignore_ascii_case(prefix))
        })
}

/// Check that a registry ID is nonempty and contains only ASCII letters, digits, `_`, or `-`.
pub fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}
