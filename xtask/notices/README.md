# Package notices

Run from the repository root:

```sh
cargo notices build
cargo notices check
cargo notices build wasm
cargo notices check node
```

`build` writes package licenses and third-party notices. `check` fails if they are missing or outdated, without rewriting them. Omit the package name to select all packages.

Supported names: `comfit`, `comfit-cli`, `comfit-core`, `comfit-grammars`, `comfit-themes`, `css`, `node`, `rehype`, `python`, `docs`, `wasm`, and `wasm-grammars`.

Grammar notices require prepared sources from `cargo grammars update`. Notice generation runs offline and does not compile parsers or WASM.

Edit package contents and component metadata in `registry/notices.toml`. Run `cargo notices build` before packaging Rust crates or Python distributions. `just lint` and npm prepack hooks run `check`.
