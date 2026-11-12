//! Compile prepared grammars, compressing package inputs only for release builds.

use std::{fs, path::Path, process::Command};

use anyhow::{Context, Result, ensure};
use comfit_internal::{command::run as run_command, grammar::Registry};
use rayon::prelude::*;

use crate::update::copy;

pub fn run(root: &Path, release: bool) -> Result<()> {
    let data = root.join("crates/comfit_grammars/data");
    let registry: Registry = toml::from_str(
        &fs::read_to_string(data.join("languages.toml"))
            .context("run cargo grammars update before building grammars")?,
    )?;
    registry.validate()?;

    let work = root.join("target/comfit-grammars");
    let staging = work.join("package-data");
    let previous = work.join("previous-data");
    ensure!(
        !previous.exists(),
        "remove or recover {} first",
        previous.display()
    );
    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    copy(&data, &staging)?;

    rayon::ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism()?.get().min(2))
        .build()?
        .install(|| {
            registry
                .grammars
                .par_iter()
                .try_for_each(|(id, grammar)| -> Result<()> {
                    if !grammar.prebuilt {
                        return Ok(());
                    }

                    let parser = root
                        .join(".grammars/prepared")
                        .join(id)
                        .join(&grammar.path)
                        .join("src/parser.c");
                    let output = staging
                        .join("grammars")
                        .join(id)
                        .join(&grammar.path)
                        .join("src/parser.c");
                    let compressed = output.with_extension("c.zst");
                    if release {
                        let input = fs::File::open(&parser).with_context(|| {
                            format!("reading {id} parser; run cargo grammars update first")
                        })?;
                        zstd::stream::copy_encode(input, fs::File::create(&compressed)?, 12)?;
                        if output.exists() {
                            fs::remove_file(output)?;
                        }
                        println!("Compressed grammar {id}");
                    } else {
                        fs::copy(&parser, &output).with_context(|| {
                            format!("reading {id} parser; run cargo grammars update first")
                        })?;
                        if compressed.exists() {
                            fs::remove_file(compressed)?;
                        }
                    }
                    Ok(())
                })
        })?;

    fs::rename(&data, &previous)?;
    if let Err(error) = fs::rename(&staging, &data) {
        fs::rename(&previous, &data)?;
        return Err(error.into());
    }
    fs::remove_dir_all(previous)?;

    let mut build = Command::new("cargo");
    build.current_dir(root).args([
        "build",
        "--locked",
        "--package",
        "comfit-grammars",
        "--all-features",
    ]);
    if release {
        build.arg("--release");
    }
    run_command(&mut build)
}
