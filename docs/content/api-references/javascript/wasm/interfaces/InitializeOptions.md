[@comfit/wasm](../index.md) / InitializeOptions

# Interface: InitializeOptions

Defined in: [types.ts:199](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L199)

WASM sources for initializing both runtimes before creating a highlighter.

## Properties

### engine

> **engine**: [`WasmSource`](../type-aliases/WasmSource.md)

Defined in: [types.ts:201](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L201)

Comfit's shared highlighting engine, built as `comfit_bg.wasm`.

---

### treeSitter

> **treeSitter**: [`WasmSource`](../type-aliases/WasmSource.md)

Defined in: [types.ts:203](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L203)

The pinned web-tree-sitter runtime WASM.
