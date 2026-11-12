//! Prepare pinned grammar sources and validate inherited query data.

use anyhow::Result;
use clap::{Parser, Subcommand};

mod build;
mod outdated;
mod query;
mod update;

#[derive(Parser)]
#[command(about = "Prepare pinned grammar sources and minified queries")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build all grammar features, compressing package inputs only with --release.
    Build {
        /// Compress prebuilt parsers at zstd level 12 and build optimized grammars.
        #[arg(long)]
        release: bool,
    },
    /// Fetch pinned sources and prepare validated, uncompressed crate inputs.
    Update {
        /// Optimize parser libraries used for validation.
        #[arg(long)]
        release: bool,
    },
    /// Compare pinned commits with upstream default-branch heads without changing pins.
    Outdated {
        /// Grammar IDs to check; omit to check all grammars.
        grammars: Vec<String>,
    },
}

fn main() -> Result<()> {
    let Args { command } = Args::parse();
    let root = comfit_internal::workspace_root();

    match command {
        Command::Build { release } => build::run(root, release),
        Command::Update { release } => update::run(root, release),
        Command::Outdated { grammars } => outdated::run(root, &grammars),
    }
}
