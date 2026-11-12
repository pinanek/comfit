# Contributing

Run commands from the repository root. Use `just` to list available recipes.

For building individual packages, see [Build from source](https://comfit.pinanek.me/docs/development/build-from-source).

## Setup

Install Rust with Cargo, C/C++ build tools, Git, `patch`, `just`, Node.js 22 or later, pnpm, Python 3.11 or later, and uv. Formatting and linting also require dprint and Tombi.

```sh
just init
cargo grammars update
just build
uv sync --package comfit --locked
```

`just init` installs build tools, workspace and docs dependencies, and Git hooks. Use `just install` to refresh dependencies without reinstalling tools.

The npm packages and docs site share the default catalog in `pnpm-workspace.yaml` and the root `pnpm-lock.yaml`. Use `--save-catalog` with `pnpm add` when adding a dependency. Workspace links and peer compatibility ranges remain explicit in package manifests.

`just build` runs native and WASM development builds concurrently. Its `build-wasm` branch generates the engine and grammar assets together, then bundles both packages without regenerating assets. Use `just build-wasm` to build only the browser packages. The Python install above also uses the development profile.

Rerun `cargo grammars update` after changing queries or the language registry. It prepares uncompressed inputs; `just build` and `just build-native` let consumer builds compile the grammars without a separate prebuild. Use `cargo grammars build` for a focused grammar-only build, or `cargo grammars build --release` to compress prebuilt package inputs at zstd level 12 and build optimized grammars before packaging the Rust grammar crate.

Rebuild affected npm packages before running their checks. Python API tests build the extension directly with Cargo; wheel installation and packaging are checked separately by `just test-packages-python`.

## Checks

After building packages, install Chromium and run checks:

```sh
pnpm exec playwright install chromium
just check
```

On Linux, use `pnpm exec playwright install --with-deps chromium`.

| Command              | Purpose                                                           |
| -------------------- | ----------------------------------------------------------------- |
| `just fmt`           | Format source files                                               |
| `just fmt-check`     | Check formatting without modifying files                          |
| `just lint`          | Lint source and check package notices                             |
| `just lint-fast`     | Lint TOML, JavaScript, and TypeScript without compiling           |
| `just typecheck`     | Check npm package and documentation types                         |
| `just test`          | Run Rust, Node, Python, and browser WASM tests                    |
| `just test-packages` | Test packed npm packages and a Python wheel in clean environments |

Use `just` to list focused checks such as `test-rust`, `test-node`, `test-python`, `test-wasm`, and `test-grammar-features`.

`just check` runs native and JavaScript checks concurrently, but keeps Cargo checks and Python extension builds serialized. Clippy uses `COMFIT_CHECK_ONLY=1` to generate grammar bindings without compiling native parsers; normal builds and tests still compile them. The combined checks build the Python extension with workspace features to reuse Rust test build outputs, without maturin's dependency-wide linker flags. Typechecks and independent API test suites also run concurrently. Use `just --jobs 1 check` to serialize recipe execution.

Before packaging, run `cargo notices build` to refresh licenses and third-party notices. Use `cargo notices check` to verify them.

To build a Python release wheel:

```sh
uv build --package comfit --wheel --out-dir dist -C 'maturin.build-args=--profile release'
```

Pass the wheel's path to `just test-packages-python` or `just test-packages` to test that exact artifact without rebuilding it.

## Documentation

Edit guides in `docs/content`. Edit API descriptions in package sources and Python stubs, not generated reference pages.

```sh
just docs-api         # Generate and format API references
cargo docs manifest   # Refresh metadata from existing references
just docs-dev         # Start the development server
just docs-build       # Build the site and search index
just docs-start       # Serve the built site
```

Use the built site to check search, which requires the generated index. See [docs/README.md](./docs/README.md) for source layout and conventions.

## Git hooks

`just init` installs pre-commit and pre-push hooks. Pre-commit checks formatting and fast lint on staged files. Pre-push runs typechecks and package notice checks for affected paths, without running API tests, workspace Clippy, or Python rebuilds.

Run `just check` for full validation before submitting changes. Hooks do not replace it.

```sh
just hooks       # Reinstall hooks
prek uninstall   # Remove hooks
```
