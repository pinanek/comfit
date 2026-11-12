[@comfit/wasm](../index.md) / InitializeOptions

# Interface: InitializeOptions

Defined in: [types.ts:199](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L199)

WASM sources for initializing both runtimes before creating a highlighter.

## Properties

### engine

> **engine**: [`WasmSource`](../type-aliases/WasmSource.md)

Defined in: [types.ts:201](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L201)

Comfit's shared highlighting engine, built as `comfit_bg.wasm`.

---

### treeSitter

> **treeSitter**: [`WasmSource`](../type-aliases/WasmSource.md)

Defined in: [types.ts:203](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L203)

The pinned web-tree-sitter runtime WASM.
