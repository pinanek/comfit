# Contributing

Run commands from the repository root. Use `just` to list available recipes.

For package-specific instructions, see [Build from source](https://comfit.pinanek.me/docs/development/build-from-source).

## Setup

Install Rust with Cargo, C/C++ build tools, Git, `patch`, `just`, Node.js 22 or later, pnpm, Python 3.11 or later, and uv. Formatting and linting also require dprint and Tombi.

```sh
just init
cargo grammars update
just build
uv sync --package comfit-hl --locked
```

`just init` installs build tools, workspace dependencies, and Git hooks. Use `just install` to refresh dependencies without reinstalling tools.

The npm packages and docs share `pnpm-workspace.yaml` and the root `pnpm-lock.yaml`. Use `--save-catalog` with `pnpm add` for shared dependency versions. Keep workspace links and peer compatibility ranges explicit in package manifests.

### Builds

`just build` prepares native and WASM assets serially, then bundles the JavaScript packages concurrently. Use `just build-native` for native npm packages or `just build-wasm` for browser packages. The Python install above also uses the development profile.

Rerun `cargo grammars update` after changing queries or the language registry. It prepares uncompressed inputs that consumer builds compile as needed. Use `cargo grammars build` for a grammar-only build. Before packaging the Rust grammar crate, use `cargo grammars build --release` to compress prebuilt inputs at zstd level 12 and build optimized grammars.

Rebuild affected npm packages before running checks. Python API tests build the extension directly with Cargo; wheel installation is checked separately by `just test-packages-python`.

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

GitHub runs CI and security checks on pull requests and pushes to `main`; use a manual workflow run to check a branch without a pull request.

`just check` runs independent checks concurrently, while serializing Cargo checks and Python extension builds to reuse build outputs. Clippy uses `COMFIT_CHECK_ONLY=1` to generate grammar bindings without compiling native parsers; builds and tests still compile them. Use `just --jobs 1 check` to run sequentially.

### Package validation

Run `cargo notices build` before packaging to refresh licenses and third-party notices. Use `cargo notices check` to verify them.

To build a Python release wheel:

```sh
uv build --package comfit-hl --wheel --out-dir dist -C 'maturin.build-args=--profile release'
```

Pass the wheel's path to `just test-packages-python` or `just test-packages` to test it without rebuilding.

## Documentation

Edit guides in `docs/content`. Edit API descriptions in package sources and Python stubs, not generated reference pages.

```sh
just docs-api         # Generate and format API references
cargo docs manifest   # Refresh metadata from existing references
just docs-dev         # Start the development server
just docs-build       # Build the site and search index
just docs-start       # Serve the built site
```

Check search on the built site, which includes the generated index. See [docs/README.md](./docs/README.md) for source layout and conventions.

## Releases

Use Conventional Commits, such as `fix: handle empty input` or `feat: add a language`. Each public package has its own version, changelog, and tag, such as `comfit-core-v0.1.0`, `node-v0.1.0`, or `comfit-hl-v0.1.0`. Release Please groups affected packages in one PR to keep dependency updates and lockfiles consistent. Private tooling and docs are not released.

`.github/scripts/release-please.mjs` determines affected packages from Cargo and npm dependencies, generated assets, and shared inputs. A Python-only fix releases only `comfit-hl`; a core engine change can also release its bindings and consumers.

Review the release PR and wait for CI before merging. Merging creates component tags and GitHub releases, which trigger registry publishing.

When adding a public package:

- Register its path in `release-please-config.json` and add an unpublished entry to `.release-please-manifest.json`.
- Use `extra-files` for companion manifests and shared dependency ranges.
- Add generated inputs that package managers cannot describe to the release script.

### Automation setup

Add a repository secret named `RELEASE_PLEASE_TOKEN`: a fine-grained personal access token with read/write access to Contents, Issues, and Pull requests. This lets release PR updates trigger CI, which GitHub's built-in `GITHUB_TOKEN` does not.

Before publishing, confirm ownership of the Rust crate names, the `@comfit` npm scope, and `comfit-hl` on PyPI. Create the GitHub environments below with required reviewers and deployment rules that allow component release tags.

| Workflow             | Environment      | Artifacts                                                                      |
| -------------------- | ---------------- | ------------------------------------------------------------------------------ |
| `publish-rust.yml`   | `crates-io`      | The matching Rust crate, verified with `cargo publish --dry-run`               |
| `publish-cli.yml`    | `github-release` | Eight CLI archives and a `SHA256SUMS` file attached to the CLI release         |
| `publish-npm.yml`    | `npm`            | The matching npm package; Node releases include eight native platform packages |
| `publish-python.yml` | `pypi`           | One `comfit-hl` sdist and seven ABI3 wheels                                    |

Configure trusted publishers with this repository, the exact workflow filename, and its environment:

- **crates.io:** Configure `publish-rust.yml` with environment `crates-io`. If the first publication requires a token, add `CARGO_REGISTRY_TOKEN` as an environment secret and remove it after configuring trusted publishing.
- **npm:** Configure `publish-npm.yml` with environment `npm` for every public npm package, including the eight Node platform packages. Packages must exist before trusted publishing can be configured. Use an environment secret named `NPM_TOKEN` with a short-lived granular token for first publications, then remove it. Later publications use OIDC and provenance.
- **PyPI:** Add a pending trusted publisher for `comfit-hl`, workflow `publish-python.yml`, and environment `pypi`. No PyPI token is needed.

### Publishing

Workflows check out the exact component tag and verify its version against the package and release manifests. Publishing runs once after builds and checks complete, never from matrix jobs.

CLI releases build eight binaries: macOS and Windows on x86_64 and ARM64, plus glibc 2.28+ and static musl Linux on both architectures. Each archive includes the executable, README, license, and third-party notices. The packaging script extracts and checks each archive; musl binaries also run in Alpine. After all builds pass, the upload job uses the `github-release` environment to attach ZIP archives for Windows, tarballs for macOS/Linux, and `SHA256SUMS` to the matching `comfit-cli-v*` release. It uses `GITHUB_TOKEN`, independently of crates.io publishing, and replaces assets with the same names on retries.

Node's eight-target matrix covers macOS, Windows, and glibc/musl Linux on x86_64 and ARM64. Generated `@comfit/node-{platform}` packages share the wrapper's version, declare OS/CPU/libc restrictions, and are exact-version optional dependencies of `@comfit/node`. The NAPI loader selects the installed binary; the wrapper ships only the loader and types. Platform packages do not need separate release PR entries. Musl builds use Zig and cargo-zigbuild, with installed-package checks in Alpine on both architectures.

Python wheels are built from the release sdist and tested on Python 3.15. The ABI3 wheels support CPython 3.11 or later. Wheels cover Linux and macOS on x86_64 and ARM64, plus Windows x86_64. Linux baselines are manylinux 2.28 and musllinux 1.2; macOS targets 11.0 or later. Musllinux wheels are tested in Alpine. The sdist includes prepared grammars, so installation does not require grammar-generation tools.

Publishing jobs cache Rust dependencies by job and target; Python wheel builds use sccache.

Each workflow accepts a `tag` input for manual retries. Rust and npm wait up to 20 minutes for local dependencies to appear in the registry; fix failed prerequisites before retrying. npm skips builds only when all packages for that release exist, including Node platform packages. Python skips builds only when its sdist and all seven wheel targets exist. Partial uploads remain retryable; publishers skip existing versions or files.

## Git hooks

`just init` installs pre-commit and pre-push hooks. Pre-commit checks formatting and fast lint on staged files. Pre-push runs affected typechecks and package notice checks, without API tests, workspace Clippy, or Python rebuilds.

Run `just check` before submitting changes; hooks do not replace full validation.

```sh
just hooks       # Reinstall hooks
prek uninstall   # Remove hooks
```
