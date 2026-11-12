//! Local include resolution and VS Code theme conversion.

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail, ensure};
use comfit_themes::convert::{VscodeTheme, convert, to_toml};
use serde::Deserialize;

use crate::cli::Convert;

/// Resolve local theme includes, convert their merged rules, and write Comfit TOML.
///
/// # Errors
///
/// Returns an error for invalid JSON/JSON5, unresolved or cyclic includes, nesting beyond
/// 64 files, unsupported theme values, or input/output failures.
pub(crate) fn run(args: &Convert) -> Result<()> {
    let source = args.input.read()?;
    #[derive(Deserialize)]
    struct Metadata {
        name: Option<String>,
    }

    let metadata: Metadata = json5::from_str(&source).context("reading theme metadata")?;
    let name = args
        .name
        .as_deref()
        .or(metadata.name.as_deref())
        .or_else(|| {
            args.input
                .path()
                .and_then(Path::file_stem)
                .and_then(|stem| stem.to_str())
        })
        .unwrap_or("Converted theme");
    let parsed = resolve(&source, args.input.path(), &mut Vec::new())?;
    let conversion = convert(name, &parsed)?;

    if args.report_unmapped {
        for (selector, count) in &conversion.unmapped_selectors {
            eprintln!("unmapped ({count}): {selector}");
        }
    }

    let toml = to_toml(&conversion.theme);
    args.input
        .write(|output| Ok(output.write_all(toml.as_bytes())?))
}

fn resolve(source: &str, path: Option<&Path>, stack: &mut Vec<PathBuf>) -> Result<VscodeTheme> {
    ensure!(stack.len() < 64, "theme include nesting exceeds 64 files");

    if let Some(path) = path {
        let canonical =
            fs::canonicalize(path).with_context(|| format!("resolving {}", path.display()))?;
        ensure!(
            !stack.contains(&canonical),
            "theme include cycle at {}",
            path.display()
        );
        stack.push(canonical);
    }

    let mut child = VscodeTheme::from_json(source).with_context(|| {
        path.map_or_else(
            || "parsing theme from stdin".into(),
            |path| format!("parsing {}", path.display()),
        )
    })?;
    let result = if let Some(include) = child.include.take() {
        let Some(path) = path else {
            bail!("cannot resolve theme includes from stdin; supply a theme file");
        };
        let included = path.parent().unwrap_or(Path::new(".")).join(include);
        let source = fs::read_to_string(&included)
            .with_context(|| format!("reading included theme {}", included.display()))?;
        let mut parent = resolve(&source, Some(&included), stack)?;
        parent.merge(child)?;
        parent
    } else {
        child
    };

    if path.is_some() {
        stack.pop();
    }
    Ok(result)
}
