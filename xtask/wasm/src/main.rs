//! Build the browser highlighting engine and separate WASM grammar assets.

use std::{collections::BTreeSet, env, fs, io::Write as _, path::Path, process::Command};

use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use clap::{Parser, Subcommand, ValueEnum};
use comfit_internal::command::run;
use comfit_internal::grammar::{QUERY_KINDS, Registry};
use etcetera::BaseStrategy;
use rayon::prelude::*;

#[derive(Parser)]
#[command(about = "Build the shared WASM engine and language assets")]
struct Args {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    Build {
        #[arg(value_enum, default_value = "all")]
        package: Package,
        /// Build optimized assets instead of faster development assets.
        #[arg(long)]
        release: bool,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Package {
    All,
    Engine,
    Grammars,
}

fn main() -> Result<()> {
    let Args { command } = Args::parse();
    let root = comfit_internal::workspace_root();

    match command {
        Action::Build { package, release } => build(root, package, release),
    }
}

fn build(root: &Path, package: Package, release: bool) -> Result<()> {
    for name in ["wasm", "wasm-grammars"] {
        if (name == "wasm" && package == Package::Grammars)
            || (name == "wasm-grammars" && package == Package::Engine)
        {
            continue;
        }

        run(Command::new("cargo")
            .current_dir(root)
            .args(["notices", "build", name]))?;
    }

    let metadata = MetadataCommand::new()
        .manifest_path(root.join("Cargo.toml"))
        .no_deps()
        .exec()?;
    let target = metadata.target_directory.as_std_path();
    let staging = target.join("comfit-wasm/generated");

    let engine = staging.join("engine");
    let grammars = staging.join("grammars");

    for (output, enabled) in [
        (&engine, package != Package::Grammars),
        (&grammars, package != Package::Engine),
    ] {
        if enabled {
            if output.exists() {
                fs::remove_dir_all(output)?;
            }

            fs::create_dir_all(output)?;
        }
    }

    match package {
        Package::All => {
            let (engine_result, grammar_result) = rayon::join(
                || build_engine(root, target, &engine, release),
                || build_grammars(root, &grammars, release),
            );
            engine_result?;
            grammar_result?;
        }
        Package::Engine => build_engine(root, target, &engine, release)?,
        Package::Grammars => build_grammars(root, &grammars, release)?,
    }

    for (source, destination, enabled) in [
        (
            engine,
            root.join("npm/wasm/js/generated"),
            package != Package::Grammars,
        ),
        (
            grammars,
            root.join("npm/wasm-grammars/generated"),
            package != Package::Engine,
        ),
    ] {
        if !enabled {
            continue;
        }

        if destination.exists() {
            fs::remove_dir_all(&destination)?;
        }

        fs::create_dir_all(
            destination
                .parent()
                .context("missing generated directory parent")?,
        )?;
        fs::rename(source, &destination)?;
        println!("Generated {}", destination.display());
    }

    Ok(())
}

fn build_engine(root: &Path, target: &Path, output: &Path, release: bool) -> Result<()> {
    let mut build = Command::new("cargo");
    build.current_dir(root).args([
        "build",
        "--locked",
        "-p",
        "comfit-wasm",
        "--target",
        "wasm32-unknown-unknown",
    ]);
    if release {
        build.arg("--release");
    }
    run(&mut build)?;

    let profile = if release { "release" } else { "debug" };
    run(Command::new("wasm-bindgen")
        .arg(target.join(format!("wasm32-unknown-unknown/{profile}/comfit_wasm.wasm")))
        .args(["--target", "web", "--out-name", "comfit", "--out-dir"])
        .arg(output))?;

    if release {
        let engine = output.join("comfit_bg.wasm");
        let optimized = output.join("comfit_bg.optimized.wasm");
        run(Command::new("wasm-opt")
            .arg(&engine)
            .args([
                "-Oz",
                "--enable-bulk-memory",
                "--enable-reference-types",
                "--enable-multivalue",
                "-o",
            ])
            .arg(&optimized))?;
        fs::rename(optimized, engine)?;
    }

    Ok(())
}

fn build_grammars(root: &Path, output: &Path, release: bool) -> Result<()> {
    let data = root.join("crates/comfit_grammars/data");
    let registry: Registry = toml::from_str(
        &fs::read_to_string(data.join("languages.toml"))
            .context("run cargo grammars update before building WASM assets")?,
    )?;
    let mut grammars: BTreeSet<_> = registry
        .languages
        .values()
        .map(|language| &language.grammar)
        .collect();

    let compiler = if release {
        None
    } else {
        let sdk = match env::var_os("TREE_SITTER_WASI_SDK_PATH") {
            Some(path) => path.into(),
            None => etcetera::choose_base_strategy()?
                .cache_dir()
                .join("tree-sitter/wasi-sdk"),
        };
        let candidates = ["clang", "wasm32-unknown-wasi-clang", "wasm32-wasi-clang"].map(|name| {
            sdk.join("bin")
                .join(format!("{name}{}", env::consts::EXE_SUFFIX))
        });
        if !candidates.iter().any(|path| path.is_file())
            && let Some(id) = grammars.first()
        {
            // Let Tree-sitter install its pinned WASI SDK on first use.
            run(Command::new("tree-sitter")
                .args(["build", "--wasm", "--output"])
                .arg(output.join(format!("tree-sitter-{id}.wasm")))
                .arg(
                    root.join(".grammars/prepared")
                        .join(id)
                        .join(&registry.grammars[*id].path),
                ))?;
            println!("Built WASM grammar {id}");
            grammars.pop_first();
        }
        Some(candidates.into_iter().find(|path| path.is_file()).context(
            "WASI compiler not found; set TREE_SITTER_WASI_SDK_PATH to the SDK directory",
        )?)
    };

    // Install Tree-sitter's shared WASI/Binaryen tools before concurrent release builds.
    if release && let Some(id) = grammars.pop_first() {
        run(Command::new("tree-sitter")
            .args(["build", "--wasm", "--output"])
            .arg(output.join(format!("tree-sitter-{id}.wasm")))
            .arg(
                root.join(".grammars/prepared")
                    .join(id)
                    .join(&registry.grammars[id].path),
            ))?;
        println!("Built WASM grammar {id}");
    }

    rayon::ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism()?.get().min(2))
        .build()?
        .install(|| {
            grammars.par_iter().try_for_each(|&id| -> Result<()> {
                let grammar = &registry.grammars[id];
                let directory = root.join(".grammars/prepared").join(id);
                let parser = output.join(format!("tree-sitter-{id}.wasm"));
                if let Some(compiler) = &compiler {
                    let source = directory.join(&grammar.path).join("src");
                    let mut build = Command::new(compiler);
                    build.current_dir(&source).args([
                        "--target=wasm32-wasip1",
                        "-fPIC",
                        "-shared",
                        "--no-wasm-opt",
                        "-g0",
                        // Ruby needs optimization to lower strchr to a supported WASM import.
                        if id == "ruby" { "-Os" } else { "-O0" },
                        "-Wl,--allow-undefined",
                        "-Wl,--no-entry",
                        "-nostdlib",
                        "-fno-exceptions",
                        "-fvisibility=hidden",
                        "-I",
                        ".",
                        "parser.c",
                    ]);
                    build
                        .arg(format!("-Wl,--export={}", grammar.symbol))
                        .arg("-o")
                        .arg(&parser);
                    for scanner in ["scanner.c", "scanner.cc", "scanner.cpp"] {
                        if source.join(scanner).is_file() {
                            build.arg(scanner);
                        }
                    }
                    run(&mut build)?;
                } else {
                    run(Command::new("tree-sitter")
                        .args(["build", "--wasm", "--output"])
                        .arg(&parser)
                        .arg(directory.join(&grammar.path)))?;
                }

                println!("Built WASM grammar {id}");
                Ok(())
            })
        })?;

    let mut descriptor = Vec::new();
    for (id, language) in &registry.languages {
        let mut definition = serde_json::json!({
            "id": id,
            "aliases": language.aliases,
        });

        for kind in QUERY_KINDS {
            definition[kind] =
                fs::read_to_string(data.join("queries").join(id).join(format!("{kind}.scm")))?
                    .into();
        }

        descriptor.clear();
        write!(
            descriptor,
            "/** Generated WASM descriptor for `{id}`; regenerate with `cargo wasm build grammars`. */\n\nimport type {{ Language }} from \"../src/types.js\";\n\n/** Parser asset and resolved syntax queries for `{id}`. */\nconst language: Language = {{ ...",
        )?;
        serde_json::to_writer(&mut descriptor, &definition)?;
        write!(
            descriptor,
            ", wasm: new URL(\"./tree-sitter-{}.wasm\", import.meta.url).href }};\n\nexport default language;\n",
            language.grammar,
        )?;
        fs::write(
            output.join(format!("{}.ts", id.replace('_', "-"))),
            &descriptor,
        )?;
    }

    Ok(())
}
