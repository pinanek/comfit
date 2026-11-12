//! Resolve query inheritance and minify Tree-sitter query sources.

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail, ensure};
use comfit_internal::valid_id;

/// Resolve inherited queries parent-first and return their minified sources.
///
/// Each query file is included once. A missing query file contributes no text.
///
/// # Errors
///
/// Returns an error for invalid language IDs, missing directories, inheritance cycles,
/// malformed string literals or inheritance entries, and file-read failures.
pub fn resolve(root: &Path, language: &str, kind: &str) -> Result<String> {
    let mut output = String::new();

    visit(
        root,
        language,
        kind,
        &mut Vec::new(),
        &mut HashSet::new(),
        &mut output,
    )?;

    Ok(output)
}

fn visit(
    root: &Path,
    language: &str,
    kind: &str,
    stack: &mut Vec<PathBuf>,
    visited: &mut HashSet<PathBuf>,
    output: &mut String,
) -> Result<()> {
    ensure!(valid_id(language), "invalid inherited language: {language}");

    let directory = root.join(language);
    ensure!(directory.is_dir(), "missing query directory: {language}");

    let path = directory.join(format!("{kind}.scm"));
    ensure!(
        !stack.contains(&path),
        "query inheritance cycle: {} -> {}",
        stack
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(" -> "),
        path.display()
    );

    if visited.contains(&path) || !path.exists() {
        return Ok(());
    }

    let source =
        fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let (minified, parents) =
        minify(&source).with_context(|| format!("reading {}", path.display()))?;

    stack.push(path);

    for parent in parents {
        let optional = parent.starts_with('(') && parent.ends_with(')');
        let parent = if optional {
            &parent[1..parent.len() - 1]
        } else {
            parent
        };

        if optional && !root.join(parent).is_dir() {
            continue;
        }

        visit(root, parent, kind, stack, visited, output)?;
    }

    visited.insert(stack.pop().unwrap());

    if !minified.is_empty() {
        if !output.is_empty() {
            output.push(' ');
        }
        output.push_str(&minified);
    }

    Ok(())
}

fn minify(source: &str) -> Result<(String, Vec<&str>)> {
    let mut output = String::new();
    let mut parents = Vec::new();
    let mut characters = source.chars();
    let mut whitespace = false;

    while let Some(character) = characters.next() {
        if character == ';' {
            let remaining = characters.as_str();
            let (comment, rest) = remaining.split_once('\n').unwrap_or((remaining, ""));
            characters = rest.chars();

            if let Some(inherited) = comment.trim().strip_prefix("inherits:") {
                parents.extend(inherited.split(',').map(str::trim));
            }

            whitespace = true;
            continue;
        }

        if character.is_whitespace() {
            whitespace = true;
            continue;
        }

        if whitespace && !output.is_empty() {
            output.push(' ');
        }

        whitespace = false;
        output.push(character);

        if character == '"' {
            loop {
                let character = characters.next().context("unterminated query string")?;
                output.push(character);

                match character {
                    '\\' => output.push(characters.next().context("unterminated query escape")?),
                    '"' => break,
                    _ => {}
                }
            }
        }
    }

    if parents.iter().any(|parent| parent.is_empty()) {
        bail!("empty query inheritance entry");
    }

    Ok((output, parents))
}
