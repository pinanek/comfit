//! Compare pinned grammar commits with upstream default-branch heads without modifying data.

use std::{collections::HashMap, fs, path::Path, process::Command};

use anyhow::{Context, Result, ensure};
use comfit_internal::grammar::Registry;

pub fn run(root: &Path, ids: &[String]) -> Result<()> {
    let registry: Registry =
        toml::from_str(&fs::read_to_string(root.join("registry/languages.toml"))?)?;
    registry.validate()?;

    for id in ids {
        ensure!(registry.grammars.contains_key(id), "unknown grammar: {id}");
    }

    let mut repositories = HashMap::new();
    let mut failures = 0;
    println!("Grammar\tCurrent\tUpstream HEAD\tStatus");

    for (id, grammar) in &registry.grammars {
        if !ids.is_empty() && !ids.contains(id) {
            continue;
        }

        let result = (|| -> Result<()> {
            if !repositories.contains_key(&grammar.repository) {
                let output = Command::new("git")
                    .env("GIT_TERMINAL_PROMPT", "0")
                    .args(["ls-remote", &grammar.repository, "HEAD"])
                    .output()
                    .context("listing upstream HEAD with git")?;
                ensure!(
                    output.status.success(),
                    "git ls-remote failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                let text = String::from_utf8(output.stdout)?;
                let head = text
                    .split_whitespace()
                    .next()
                    .context("missing upstream HEAD")?;
                repositories.insert(grammar.repository.clone(), head.to_owned());
            }
            let head = &repositories[&grammar.repository];
            let status = if head == &grammar.revision {
                "up to date"
            } else {
                "upstream differs; needs compatibility validation"
            };
            println!("{id}\t{}\t{head}\t{status}", grammar.revision);
            Ok(())
        })();

        if let Err(error) = result {
            eprintln!("{id}: {error:#}");
            failures += 1;
        }
    }

    ensure!(failures == 0, "failed to check {failures} grammar(s)");
    Ok(())
}
