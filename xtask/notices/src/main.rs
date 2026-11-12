//! Generate and check package license files and bundled third-party notices.

use std::{collections::BTreeMap, fs};

use anyhow::{Context, Result, bail, ensure};
use clap::{Parser, Subcommand};

mod config;
mod notice;

use config::Config;

#[derive(Parser)]
#[command(about = "Generate or check package licenses and third-party notices offline")]
struct Args {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Write license files and notices for all packages or one package.
    Build { package: Option<String> },
    /// Fail if generated license files or notices are missing or outdated.
    Check { package: Option<String> },
}

fn main() -> Result<()> {
    let Args { command } = Args::parse();
    let root = comfit_internal::workspace_root();
    let config: Config = toml::from_str(&fs::read_to_string(root.join("registry/notices.toml"))?)?;
    let (check, selected) = match command {
        Action::Build { package } => (false, package),
        Action::Check { package } => (true, package),
    };
    let packages: Vec<_> = config
        .packages
        .iter()
        .filter(|package| selected.as_ref().is_none_or(|name| name == &package.name))
        .collect();
    ensure!(!packages.is_empty(), "unknown package: {selected:?}");

    let license =
        fs::read_to_string(root.join("LICENSE")).context("reading Comfit's MIT license")?;
    ensure!(!license.trim().is_empty(), "empty root LICENSE");
    let mut outputs = BTreeMap::new();

    for package in packages {
        let directory = root.join(&package.directory);
        outputs.insert(directory.join("LICENSE"), license.clone());

        if !package.components.is_empty() {
            let notices = notice::generate(root, &config, package)?;
            outputs.insert(
                directory.join(
                    package
                        .notices
                        .as_deref()
                        .unwrap_or("THIRD_PARTY_NOTICES.txt"),
                ),
                notices,
            );
        }
    }

    let mut outdated = Vec::new();

    // Resolve every input before writing so missing licenses leave existing outputs intact.
    for (path, content) in outputs {
        match fs::read_to_string(&path) {
            Ok(existing) if existing == content => continue,
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error).with_context(|| format!("reading {}", path.display())),
        }

        if check {
            outdated.push(path);
        } else {
            fs::create_dir_all(path.parent().context("missing output parent")?)?;
            fs::write(&path, content)?;
            println!(
                "Generated {}",
                path.strip_prefix(root)?
                    .to_string_lossy()
                    .replace('\\', "/")
            );
        }
    }

    if !outdated.is_empty() {
        for path in outdated {
            eprintln!(
                "Missing or outdated: {}",
                path.strip_prefix(root)?
                    .to_string_lossy()
                    .replace('\\', "/")
            );
        }

        bail!("run cargo notices build to regenerate package notices");
    }

    Ok(())
}
