# API reference generation

Run from the repository root after `just init`, `cargo grammars update`, and `cargo grammars build`:

```sh
just docs-api
just docs-api --skip-build
cargo docs manifest
```

- `just docs-api` generates and formats JavaScript and Python references in `docs/content/api-references`.
- `--skip-build` reuses existing Node declarations and WASM bindings.
- `cargo docs manifest` refreshes navigation and metadata from existing references without rebuilding them.

Use `cargo docs build` to generate references without formatting. Authored pages such as `rust.md` are preserved.
