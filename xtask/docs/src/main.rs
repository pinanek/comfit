//! Generate public JavaScript and Python API references.

use std::{fs, path::Path, process::Command};

use anyhow::{Result, ensure};
use clap::{Parser, Subcommand};
use comfit_internal::command::run;

mod manifest;

#[derive(Parser)]
#[command(about = "Generate Comfit API references")]
struct Args {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Generate JavaScript and Python API references in docs/content/api-references.
    Build {
        /// Reuse existing Node declarations and WASM bindings without building packages.
        #[arg(long)]
        skip_build: bool,
    },
    /// Refresh navigation and page metadata from existing API references.
    Manifest,
}

fn main() -> Result<()> {
    let Args { command } = Args::parse();
    let root = comfit_internal::workspace_root();

    match command {
        Action::Build { skip_build } => build(root, skip_build),
        Action::Manifest => manifest::generate(&root.join("docs/content/api-references")),
    }
}

fn build(root: &Path, skip_build: bool) -> Result<()> {
    if skip_build {
        for file in [
            "npm/node/dist/index.d.ts",
            "npm/wasm/js/generated/comfit.d.ts",
        ] {
            ensure!(root.join(file).is_file(), "missing API input: {file}");
        }
    }

    let staging = root.join("target/comfit-docs/generated");

    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    fs::create_dir_all(&staging)?;

    if !skip_build {
        run(Command::new("pnpm")
            .current_dir(root)
            .args(["--dir", "npm/node", "build"]))?;

        if !root.join("npm/wasm/js/generated/comfit.d.ts").is_file() {
            run(Command::new("cargo")
                .current_dir(root)
                .args(["wasm", "build", "engine"]))?;
        }
    }

    for (name, entry) in [
        ("node", "npm/node/dist/index.d.ts"),
        ("wasm", "npm/wasm/js/index.ts"),
        ("rehype", "npm/rehype/src/index.ts"),
        ("markdown-it", "npm/markdown-it/src/index.ts"),
        ("wasm-grammars", "npm/wasm-grammars/src/types.ts"),
    ] {
        run(Command::new("pnpm")
            .current_dir(root)
            .args([
                "exec",
                "typedoc",
                "--plugin",
                "typedoc-plugin-markdown",
                "--tsconfig",
                "docs/tsconfig.api.json",
                "--entryPoints",
                entry,
                "--name",
                &format!("@comfit/{name}"),
                "--readme",
                "none",
                "--entryFileName",
                "index.md",
                "--hidePageHeader",
                "--out",
            ])
            .arg(staging.join("javascript").join(name)))?;
    }

    run(Command::new("uv")
        .current_dir(root)
        .args([
            "run",
            "--only-group",
            "docs",
            "--locked",
            "python",
            "xtask/docs/src/python.py",
        ])
        .arg(staging.join("python")))?;

    manifest::generate(&staging)?;

    let output = root.join("docs/content/api-references");
    fs::create_dir_all(&output)?;

    for directory in ["javascript", "python"] {
        let destination = output.join(directory);
        if destination.exists() {
            fs::remove_dir_all(&destination)?;
        }
        fs::rename(staging.join(directory), destination)?;
    }
    fs::rename(staging.join("manifest.json"), output.join("manifest.json"))?;
    fs::remove_dir(staging)?;
    println!("Generated API references in {}", output.display());

    Ok(())
}
