# Grammar generation

Run from the repository root:

```sh
cargo grammars update
cargo grammars update --release
cargo grammars build
cargo grammars build --release
cargo grammars outdated
cargo grammars outdated rust javascript
```

- `update` prepares pinned parsers from `registry/languages.toml` and validates queries from `queries`. It writes uncompressed Cargo inputs to `crates/comfit_grammars/data` and WASM build sources to `.grammars/prepared`. It does not compress parsers.
- `update --release` optimizes parser libraries used for validation. The default uses faster debug builds; generated package sources are the same.
- `build` builds `comfit-grammars` with all language features using uncompressed prebuilt parsers. It restores raw parsers if a previous release build compressed the package inputs.
- `build --release` compresses prebuilt package parsers at zstd level 12, removes their raw C sources from the package inputs, and builds with Cargo's release profile. Run it before packaging. Prepared WASM sources remain uncompressed.
- `outdated` compares pins with upstream default-branch heads. Omit grammar IDs to check all grammars. It does not change pins.

Downloads run with up to eight workers after deduplicating repository revisions. Completed checkouts feed a bounded queue so generation starts before all downloads finish. Generation keeps shared checkouts serialized and uses up to two workers; query validation uses up to four. Package compression uses up to two workers. Native compilation also uses up to two workers, acquiring spare Cargo jobserver tokens so `cargo build --jobs 1` stays serial.

Requires Git, network access, C/C++ build tools, `patch`, and the Tree-sitter CLI installed by `just init`. Grammars without JSON definitions also require Node.js.

Set `prebuilt = true` in `registry/languages.toml` to package a compressed parser instead of generating it during Cargo builds. Rerun `cargo grammars update` after changing the registry or queries, then rebuild affected consumers. `cargo grammars build` is available for a focused grammar-only build; aggregate builds do not run it separately.

See [comfit-grammars](../../crates/comfit_grammars/README.md) for Cargo usage and [WASM generation](../wasm/README.md) for browser assets.
