//! Run external build tools and report command failures.

use std::process::Command;

use anyhow::{Context, Result, ensure};

/// Run a command with output sent directly to stdout and stderr.
///
/// # Errors
///
/// Returns an error if the command cannot start or exits unsuccessfully.
pub fn run(command: &mut Command) -> Result<()> {
    let status = command
        .status()
        .with_context(|| format!("running {command:?}"))?;
    ensure!(status.success(), "{command:?} failed with {status}");

    Ok(())
}

/// Run a command quietly and include its output if it fails.
///
/// # Errors
///
/// Returns an error if the command cannot start or exits unsuccessfully.
pub fn run_quiet(command: &mut Command) -> Result<()> {
    let output = command
        .output()
        .with_context(|| format!("running {command:?}"))?;
    ensure!(
        output.status.success(),
        "{command:?} failed:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(())
}
