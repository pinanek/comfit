_default:
    @just --list -u

# Set up build tools, dependencies, and Git hooks.
init: tools install hooks

# Install native and WebAssembly build tools.
tools:
    rustup target add wasm32-unknown-unknown
    just _cargo-install tree-sitter-cli --version 0.27.2 --locked
    just _cargo-install wasm-bindgen-cli --version 0.2.130 --locked
    just _cargo-install wasm-opt --version 0.116.1

_cargo-install +args:
    @if command -v cargo-binstall >/dev/null 2>&1; then \
        cargo binstall --no-confirm {{ args }}; \
    else \
        cargo install {{ args }}; \
    fi

# Install locked workspace and docs dependencies.
install:
    pnpm install --frozen-lockfile
    # Keep an existing native Python install without building it during setup.
    uv sync --all-groups --locked --no-install-workspace --inexact

# Install Git hooks.
hooks:
    just _cargo-install prek --version 0.5.10 --locked
    prek install

# Build the grammar crate from uncompressed prepared inputs.
build-grammars:
    cargo grammars build

# Build npm packages after preparing grammars.
[parallel]
build: build-native build-wasm

build-native:
    pnpm --filter @comfit/node --filter @comfit/rehype --filter @comfit/markdown-it --filter @comfit/css -r build

# Generate WASM assets concurrently, then bundle both packages.
build-wasm:
    cargo wasm build
    pnpm --filter @comfit/wasm --filter @comfit/wasm-grammars -r bundle

# Generate and format API references.
docs-api *args:
    cargo docs build {{ args }}
    dprint fmt docs/content/api-references

# Start the docs development server.
docs-dev:
    pnpm --dir docs dev

# Build docs and the search index.
docs-build:
    pnpm --dir docs build

# Serve built docs, including search.
docs-start:
    pnpm --dir docs start

# Format source files.
fmt:
    dprint fmt

# Check formatting without modifying files.
fmt-check:
    dprint check

# Lint without compiling packages or preparing grammars.
lint-fast:
    tombi lint
    pnpm exec oxlint
    pnpm exec oxlint -c docs/oxlint.config.ts docs

lint-rust:
    COMFIT_CHECK_ONLY=1 cargo lint

notices-check:
    cargo notices check

_lint-native: lint-rust notices-check

# Lint source and check package notices.
[parallel]
lint: lint-fast _lint-native

typecheck-node:
    pnpm --filter @comfit/rehype --filter @comfit/markdown-it -r typecheck

typecheck-wasm:
    pnpm --dir npm/wasm typecheck
    pnpm --dir npm/wasm-grammars typecheck

typecheck-docs:
    pnpm --dir docs exec tsc --noEmit

# Check npm package and docs types.
[parallel]
typecheck: typecheck-node typecheck-wasm typecheck-docs

test-rust:
    cargo test --workspace --all-features

test-grammar-features:
    cargo test -p comfit-grammars --no-default-features
    cargo test -p comfit-grammars --no-default-features --features rust
    cargo test -p comfit-grammars --no-default-features --features markdown

test-node:
    pnpm --filter @comfit/node --filter @comfit/rehype -r test

test-python args="":
    uv run --no-project python tests/run-python.py {{args}}

test-wasm:
    pnpm --dir npm/wasm test
    pnpm exec playwright test

_test-native: test-rust (test-python "--workspace")

# Run API tests after building packages.
[parallel]
test: _test-native test-node test-wasm

test-packages-npm:
    node tests/package-smoke.mjs npm

test-packages-python wheel="":
    uv run --no-project python tests/package-python.py --wheel {{quote(wheel)}}

# Test package archives in clean environments, optionally using an existing wheel.
test-packages wheel="": test-packages-npm (test-packages-python wheel)

_check-native: lint-rust notices-check test-rust (test-python "--workspace")

_check-js: lint-fast typecheck test-node test-wasm

# Run checks after preparing grammars and building packages.
[parallel]
check: fmt-check _check-native _check-js
