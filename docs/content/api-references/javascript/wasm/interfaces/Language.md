[@comfit/wasm](../index.md) / Language

# Interface: Language

Defined in: [types.ts:10](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L10)

A language's parser asset, ID, aliases, and Tree-sitter queries.

## Properties

### aliases?

> `readonly` `optional` **aliases?**: readonly `string`[]

Defined in: [types.ts:14](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L14)

Additional names accepted by the highlighter. Aliases and IDs share one namespace.

---

### highlights

> `readonly` **highlights**: `string`

Defined in: [types.ts:18](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L18)

Token capture query. An empty string disables this pass.

---

### id

> `readonly` **id**: `string`

Defined in: [types.ts:12](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L12)

Canonical language ID, unique within one highlighter.

---

### injections?

> `readonly` `optional` **injections?**: `string`

Defined in: [types.ts:20](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L20)

Embedded-language query. Omitted or empty queries disable this pass.

---

### locals?

> `readonly` `optional` **locals?**: `string`

Defined in: [types.ts:22](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L22)

Lexical scope, definition, and reference query.

---

### rainbows?

> `readonly` `optional` **rainbows?**: `string`

Defined in: [types.ts:24](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L24)

Query for brackets and nesting scopes, used to calculate bracket depths.

---

### wasm

> `readonly` **wasm**: [`WasmSource`](../type-aliases/WasmSource.md)

Defined in: [types.ts:16](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L16)

Parser WASM URL or bytes. Languages can share the same parser asset.
