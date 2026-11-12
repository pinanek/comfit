//! Import VS Code themes using representative TextMate scope stacks.

use std::{collections::HashMap, fs, time::Duration};

use anyhow::{Context, Result, ensure};
use clap::{Parser, Subcommand};
use comfit_internal::theme::Registry;
use comfit_themes::convert::to_toml;
use reqwest::blocking::Client;

mod import;

#[derive(Parser)]
#[command(about = "Import registered VS Code themes into Comfit TOML")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Download and convert all themes, or a single theme by ID.
    Update {
        #[arg(long)]
        theme: Option<String>,
        /// Display unmapped selectors and occurrence counts.
        #[arg(long)]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let Args { command } = Args::parse();
    let Command::Update { theme, debug } = command;
    let root = comfit_internal::workspace_root();
    let registry: Registry =
        toml::from_str(&fs::read_to_string(root.join("registry/themes.toml"))?)?;
    registry.validate()?;

    if let Some(id) = &theme {
        ensure!(
            registry.themes.iter().any(|source| &source.id == id),
            "unknown theme: {id}"
        );
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()?;
    let mut downloads = HashMap::new();
    let output = root.join("crates/comfit_themes/data");
    fs::create_dir_all(&output)?;

    for source in registry
        .themes
        .iter()
        .filter(|source| theme.as_ref().is_none_or(|id| id == &source.id))
    {
        let converted = import::run(&client, &mut downloads, source, debug)
            .with_context(|| format!("importing {}", source.id))?;
        fs::write(
            output.join(format!("{}.toml", source.id)),
            to_toml(&converted),
        )?;
        println!("Imported {}", source.id);
    }

    Ok(())
}
