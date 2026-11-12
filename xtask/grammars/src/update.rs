//! Fetch pinned grammars and stage validated parser sources and queries.

use std::{
    collections::HashMap,
    fs,
    io::{self, Cursor},
    path::{Component, Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

use anyhow::{Context, Result, ensure};
use comfit_internal::{
    command::run_quiet,
    grammar::{Grammar, QUERY_KINDS, Registry},
    is_license_filename,
};
use rayon::prelude::*;
use reqwest::blocking::Client;
use zip::ZipArchive;

use crate::query;

/// Fetch pinned parsers and validate all generated queries before replacing grammar data.
///
/// # Errors
///
/// Returns an error for invalid registry data, downloads, unsafe archive paths, external
/// tool failures, query validation, or filesystem operations.
pub fn run(root: &Path, release: bool) -> Result<()> {
    let registry: Registry =
        toml::from_str(&fs::read_to_string(root.join("registry/languages.toml"))?)?;
    registry.validate()?;

    run_quiet(Command::new("tree-sitter").arg("--version"))
        .context("install tree-sitter-cli before running cargo grammars update (see just init)")?;

    let work = root.join("target/comfit-grammars");
    let staging = work.join("data");
    let grammars = work.join("grammars");

    for directory in [&staging, &grammars] {
        if directory.exists() {
            fs::remove_dir_all(directory)?;
        }

        fs::create_dir_all(directory)?;
    }
    fs::write(work.join("empty"), "")?;

    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()?;
    let mut repositories = HashMap::new();
    for (id, grammar) in &registry.grammars {
        repositories
            .entry((&grammar.repository, grammar.revision.as_str()))
            .or_insert((id, grammar));
    }

    // Separate pools keep a full handoff queue from starving preparation workers.
    let download_pool = rayon::ThreadPoolBuilder::new().num_threads(8).build()?;
    let preparation_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread::available_parallelism()?.get().min(2))
        .build()?;
    let (sender, receiver) = mpsc::sync_channel(8);
    let cancelled = AtomicBool::new(false);
    thread::scope(|scope| -> Result<()> {
        let cancelled = &cancelled;
        let downloads = scope.spawn(move || {
            download_pool.install(|| {
                repositories
                    .par_iter()
                    .try_for_each_with(sender, |sender, (key, (id, grammar))| {
                        if cancelled.load(Ordering::Relaxed) {
                            return Ok(());
                        }
                        let path = checkout(root, &client, grammar)
                            .with_context(|| format!("fetching grammar {id}"))
                            .inspect_err(|_| cancelled.store(true, Ordering::Relaxed))?;
                        sender
                            .send((*key, path))
                            .map_err(|_| anyhow::anyhow!("grammar preparation stopped"))
                    })
            })
        });
        let prepared = preparation_pool.install(|| {
            receiver
                .into_iter()
                .par_bridge()
                .try_for_each(|(key, upstream)| -> Result<()> {
                    if cancelled.load(Ordering::Relaxed) {
                        return Ok(());
                    }
                    // Grammars from one checkout share files used by the generator.
                    for (id, grammar) in &registry.grammars {
                        if (&grammar.repository, grammar.revision.as_str()) != key {
                            continue;
                        }
                        let destination = grammars.join(id);
                        let source = upstream.join(&grammar.path);

                        let mut generate = Command::new("tree-sitter");
                        generate
                            .arg("generate")
                            .args(["--abi", "15"])
                            .current_dir(&source);
                        if source.join("src/grammar.json").is_file() {
                            generate.arg("src/grammar.json");
                        }
                        run_quiet(&mut generate).with_context(|| {
                            format!("generating parser sources for grammar {id}")
                        })?;

                        copy(
                            &source.join("src"),
                            &destination.join(&grammar.path).join("src"),
                        )?;

                        // TypeScript scanners include ../../common/scanner.h.
                        if upstream.join("common").is_dir() {
                            copy(&upstream.join("common"), &destination.join("common"))?;
                        }

                        let patch = root.join("registry/patches").join(format!("{id}.patch"));
                        if patch.is_file() {
                            run_quiet(
                                Command::new("patch")
                                    .args(["--batch", "-p1", "-i"])
                                    .arg(&patch)
                                    .current_dir(&destination),
                            )
                            .with_context(|| format!("patching sources for grammar {id}"))?;
                        }

                        copy_licenses(&upstream, &destination)?;

                        if grammar.path != "." {
                            copy_licenses(&source, &destination.join(&grammar.path))?;
                        }

                        prepare_native(
                            &destination,
                            &staging.join("grammars").join(id),
                            &destination.join(&grammar.path).join("src/parser.c"),
                            grammar.prebuilt,
                        )?;

                        let library =
                            work.join(format!("{id}.{}", std::env::consts::DLL_EXTENSION));
                        let mut build = Command::new("tree-sitter");
                        build
                            .arg("build")
                            .arg("--output")
                            .arg(&library)
                            .arg(destination.join(&grammar.path));
                        if !release {
                            build.arg("--debug");
                        }
                        run_quiet(&mut build).with_context(|| {
                            format!("validating generated sources for grammar {id}")
                        })?;
                        println!("Prepared grammar {id}");
                    }
                    Ok(())
                })
        });
        if prepared.is_err() {
            cancelled.store(true, Ordering::Relaxed);
        }
        let downloaded = downloads.join().expect("grammar download worker panicked");
        prepared?;
        downloaded
    })?;

    let queries = root.join("queries");

    rayon::ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism()?.get().min(4))
        .build()?
        .install(|| {
            registry
                .languages
                .par_iter()
                .try_for_each(|(id, language)| -> Result<()> {
                    let destination = staging.join("queries").join(id);
                    fs::create_dir_all(&destination)?;
                    let grammar = &registry.grammars[&language.grammar];
                    let library = work.join(format!(
                        "{}.{}", language.grammar, std::env::consts::DLL_EXTENSION,
                    ));

                    for kind in QUERY_KINDS {
                        let source = query::resolve(&queries, id, kind)
                            .with_context(|| format!("resolving {id}/{kind}.scm"))?;
                        let output = destination.join(format!("{kind}.scm"));
                        fs::write(&output, source)?;

                        run_quiet(
                            Command::new("tree-sitter")
                                .arg("query")
                                .arg("--quiet")
                                .arg("--lib-path")
                                .arg(&library)
                                .arg("--lang-name")
                                .arg(grammar.symbol.strip_prefix("tree_sitter_").unwrap())
                                .arg(&output)
                                .arg(work.join("empty")),
                        )
                        .with_context(|| {
                            format!(
                                "validating queries/{id}/{kind}.scm and its inherited queries against {}",
                                language.grammar
                            )
                        })?;
                    }

                    println!("Prepared queries for {id}");
                    Ok(())
                })
        })?;

    fs::write(
        staging.join("languages.toml"),
        toml::to_string_pretty(&registry)?,
    )?;

    let outputs = [
        (
            grammars,
            root.join(".grammars/prepared"),
            work.join("previous-grammars"),
        ),
        (
            staging,
            root.join("crates/comfit_grammars/data"),
            work.join("previous-data"),
        ),
    ];
    for (_, output, previous) in &outputs {
        ensure!(
            !previous.exists(),
            "remove or recover {} first",
            previous.display()
        );
        fs::create_dir_all(output.parent().context("missing output parent")?)?;
    }

    let mut installed = 0;
    let result = (|| -> Result<()> {
        for (index, (staging, output, previous)) in outputs.iter().enumerate() {
            if output.exists() {
                fs::rename(output, previous)?;
            }

            if let Err(error) = fs::rename(staging, output) {
                if previous.exists() {
                    fs::rename(previous, output)?;
                }
                return Err(error.into());
            }
            installed = index + 1;
        }
        Ok(())
    })();

    if let Err(error) = result {
        for (staging, output, previous) in outputs[..installed].iter().rev() {
            fs::rename(output, staging)?;
            if previous.exists() {
                fs::rename(previous, output)?;
            }
        }
        return Err(error);
    }

    for (_, output, previous) in &outputs {
        if previous.exists() {
            fs::remove_dir_all(previous)?;
        }
        println!("Generated {}", output.display());
    }

    Ok(())
}

fn checkout(root: &Path, client: &Client, grammar: &Grammar) -> Result<PathBuf> {
    let repository = grammar
        .repository
        .strip_prefix("https://github.com/")
        .context("grammar repositories must use https://github.com/owner/repository")?;
    let (owner, name) = repository
        .split_once('/')
        .with_context(|| format!("invalid GitHub repository: {repository}"))?;
    ensure!(
        [owner, name].into_iter().all(|part| {
            !part.is_empty()
                && !matches!(part, "." | "..")
                && part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
        }),
        "invalid GitHub repository: {repository}"
    );

    let cache = root.join(".grammars/repositories").join(repository);
    let key = &grammar.revision;
    let destination = cache.join(key);

    if destination.is_dir() {
        return Ok(destination);
    }

    let temporary = cache.join(format!("{key}.tmp"));

    if temporary.exists() {
        fs::remove_dir_all(&temporary)?;
    }

    fs::create_dir_all(&temporary)?;
    let url = format!("https://codeload.github.com/{repository}/zip/{key}");
    let bytes = client.get(url).send()?.error_for_status()?.bytes()?;
    let mut archive = ZipArchive::new(Cursor::new(bytes))?;

    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let path = entry.enclosed_name().context("unsafe archive path")?;
        let mut components = path.components();
        components.next();
        let relative = components.as_path();

        ensure!(
            relative
                .components()
                .all(|part| matches!(part, Component::Normal(_) | Component::CurDir)),
            "unsafe archive path after removing repository prefix"
        );

        if relative.as_os_str().is_empty() {
            continue;
        }

        ensure!(
            entry
                .unix_mode()
                .is_none_or(|mode| mode & 0o170000 != 0o120000),
            "symlinks are not supported in grammar archives"
        );

        let path = temporary.join(relative);

        if entry.is_dir() {
            fs::create_dir_all(&path)?;
        } else {
            fs::create_dir_all(path.parent().context("missing archive parent")?)?;
            io::copy(&mut entry, &mut fs::File::create(path)?)?;
        }
    }

    fs::rename(temporary, &destination)?;

    Ok(destination)
}

pub(super) fn copy(source: &Path, destination: &Path) -> Result<()> {
    if source.is_dir() {
        fs::create_dir_all(destination)?;
        let mut entries = fs::read_dir(source)?.collect::<io::Result<Vec<_>>>()?;
        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            copy(&entry.path(), &destination.join(entry.file_name()))?;
        }
    } else {
        fs::create_dir_all(destination.parent().context("missing destination parent")?)?;
        fs::copy(source, destination)?;
    }

    Ok(())
}

fn copy_licenses(source: &Path, destination: &Path) -> Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let name = entry.file_name();
        if is_license_filename(&name.to_string_lossy()) {
            copy(&entry.path(), &destination.join(name))?;
        }
    }

    Ok(())
}

fn prepare_native(source: &Path, destination: &Path, parser: &Path, prebuilt: bool) -> Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let output = destination.join(entry.file_name());
        if path.is_dir() {
            prepare_native(&path, &output, parser, prebuilt)?;
            continue;
        }

        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path == parser {
            if prebuilt {
                fs::copy(&path, &output)?;
            }
        } else if name == "grammar.json" {
            if !prebuilt {
                fs::copy(&path, &output)?;
            }
        } else if matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("c" | "cc" | "cpp" | "h" | "hpp" | "inc")
        ) || is_license_filename(&name)
        {
            fs::copy(&path, &output)?;
        }
    }
    Ok(())
}
