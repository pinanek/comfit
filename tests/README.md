# Public package tests

Prepare grammars and build packages with `cargo grammars update` and `just build`. Install the browser runtime with `pnpm exec playwright install chromium`, or `pnpm exec playwright install --with-deps chromium` on Linux.

- `just test` runs Rust workspace tests and documentation examples, Node API tests, Python API tests, and browser WASM tests. Python API tests load the Cargo-built extension directly; combined checks use the workspace feature graph to share Rust build outputs. `just test-python` builds only the Python package, while `just test-python --workspace` matches the combined checks.
- `just test-packages` packs all six npm packages, installs them outside the checkout, verifies their exports and TypeScript declarations, and reruns the Node API tests. It also builds a Python wheel, installs it into a fresh virtual environment, and runs the Python API tests in isolated mode.

Run `just test-packages-npm` or `just test-packages-python` to check one ecosystem. The Python wheel check uses uv and Python without requiring Node.js.

To test an already-built release wheel without rebuilding it:

```sh
just test-packages-python path/to/existing.whl
# Or include the npm package checks:
just test-packages path/to/existing.whl
```

The wheel is installed into a fresh virtual environment and tested in isolated mode, just like a newly built wheel. The supplied file is not repacked or modified, so release automation can publish that exact tested artifact. Without a wheel argument, both recipes retain their build-and-test behavior. Missing files or non-wheel paths fail rather than falling back to a rebuild.

The distribution checks use the current host's native binary and wheel. CI runs them on Linux. They require access to the npm registry and Python build dependencies.

## Shared fixtures

`crates/comfit/tests/fixtures/highlighting.json` is consumed by Rust, Node, WASM in Node and Chromium, and Python. Keeping it inside the Rust crate also includes it in the crate's source distribution.

Each fixture specifies source, language, highlighting options, and expected tokens grouped by line. A token is `[text, capture, rainbowDepth]`, with `null` for missing captures or depths. Line numbers and emphasis are checked separately. Rejoining token text with newline separators must reproduce the source exactly.

Expected captures and depths are checked in rather than generated during tests. When changing queries, review fixture differences against the intended highlighting behavior instead of accepting new output automatically.

The Rust fixture test requires the `rust`, `javascript`, `markdown`, and `markdown_inline` features. `just test` enables all features.
