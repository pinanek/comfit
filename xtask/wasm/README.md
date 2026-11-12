# WASM generation

Run from the repository root:

```sh
cargo wasm build                    # Both packages, fast development build
cargo wasm build engine             # @comfit/wasm only
cargo wasm build grammars           # @comfit/wasm-grammars only
cargo wasm build --release          # Both packages, optimized for distribution
cargo wasm build engine --release   # Optimized engine only
cargo wasm build grammars --release # Optimized grammars only
```

Run `just init` to install build tools and `cargo grammars update` before building grammar assets. Engine-only builds do not require prepared grammars. Tree-sitter may download its WASI SDK and Binaryen tools on first use. Set `TREE_SITTER_WASI_SDK_PATH` to use an existing SDK.

The engine and grammars build concurrently when both are requested. Grammar compilation uses up to two workers, after shared tool installation completes.

Generated files go to `npm/wasm/js/generated` and `npm/wasm-grammars/generated`. After generation, bundle JavaScript and copy assets into `dist` without regenerating them:

```sh
pnpm --filter @comfit/wasm --filter @comfit/wasm-grammars -r bundle
```

`just build-wasm` runs combined generation followed by bundling. `just build` also runs the native package builds concurrently with this WASM branch. Individual `pnpm --dir npm/wasm build` and `pnpm --dir npm/wasm-grammars build` commands still generate and bundle their own package.
