---
title: "Build from source"
description: "Build and install Comfit's Rust, command-line, JavaScript, Python, and WebAssembly packages from a source checkout."
---

# Build from source

Use this guide to build and install Comfit packages from a source checkout. Run commands from the repository root, and follow the sections for the packages you need.

## Prerequisites

Install Rust with Cargo, C/C++ build tools, Git, and `patch`. Some grammars also require Node.js 22 or later to generate parser sources.

Additional requirements depend on the package:

- Node.js, rehype, markdown-it, and browser packages require Node.js 22 or later and pnpm.
- The Python package requires Python 3.11 or later. The commands below use uv.
- Browser packages require the WebAssembly tools listed in their section below.

## Prepare the grammars

Install the pinned Tree-sitter CLI and prepare the parser sources and queries:

```sh
cargo install tree-sitter-cli --version 0.27.2 --locked
cargo grammars update
```

`cargo grammars update` downloads pinned parser sources, resolves query inheritance, and validates the generated data without compressing it. Consumer builds compile the required grammars from `crates/comfit_grammars/data`; `cargo grammars build` is an optional grammar-only build. Use `cargo grammars build --release` before packaging the Rust grammar crate to compress prebuilt parsers at zstd level 12 and build optimized grammars. Later Cargo builds use these local files without downloading grammars. The sources in `.grammars/prepared` remain uncompressed for WASM builds.

## Rust

```sh
cargo build -p comfit --release --features rust,render-html
```

Use language features to select the parsers included in the build. Use `all-languages` instead of `rust` to include every bundled grammar. Add `render-ansi` for terminal output or `theme-toml` to parse custom theme files.

## Command line

Build and install the `comfit` command:

```sh
cargo install --path crates/comfit_cli --locked
comfit ansi --list-languages
```

You can also run it directly from the checkout:

```sh
cargo run -p comfit-cli --release -- ansi crates/comfit/src/lib.rs --theme catppuccin-mocha
```

## Node.js and rehype

Install JavaScript dependencies and build the packages:

```sh
pnpm install --frozen-lockfile
pnpm --dir npm/node build:release
pnpm --dir npm/rehype build
pnpm --dir npm/markdown-it build
```

The Node build produces the current platform's addon and JavaScript entry point in `npm/node/dist`. The Markdown plugins write their output to `npm/rehype/dist` and `npm/markdown-it/dist`.

To install the builds in another application, create package archives:

```sh
pnpm --dir npm/node pack
pnpm --dir npm/rehype pack
pnpm --dir npm/markdown-it pack
```

Install the resulting `.tgz` files in your application. For either Markdown plugin, install its archive together with the Node archive so it uses your Node package build.

## Browser packages

Install the WebAssembly target and pinned build tools:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130 --locked
cargo install wasm-opt --version 0.116.1
```

Then build the engine, grammar assets, and theme stylesheets:

```sh
pnpm install --frozen-lockfile
cargo wasm build
pnpm --filter @comfit/wasm --filter @comfit/wasm-grammars -r bundle
pnpm --dir npm/css build
```

Each package writes its output to its own `dist` directory. The engine and grammar assets generate concurrently. Add `--release` to `cargo wasm build` for optimized assets. Emscripten is not required because Tree-sitter uses its WASI toolchain.

## Python

Build and install the extension into the checkout's Python environment:

```sh
uv sync --package comfit-hl --locked
uv run --package comfit-hl python -c 'import comfit; print(comfit.languages())'
```

To build and install into an existing Python environment instead, run `python -m pip install ./python/comfit` after preparing the grammars.

These Python builds use the development profile. To build a release wheel:

```sh
uv build --package comfit-hl --wheel --out-dir dist -C 'maturin.build-args=--profile release'
```
