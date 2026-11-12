[@comfit/wasm](../index.md) / Token

# Interface: Token

Defined in: [types.ts:131](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L131)

Owned source text with an optional capture style and bracket depth.

## Properties

### highlight

> `readonly` **highlight**: `string` \| `null`

Defined in: [types.ts:135](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L135)

Query capture name, or null for unstyled text.

---

### rainbow

> `readonly` **rainbow**: `number` \| `null`

Defined in: [types.ts:137](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L137)

Zero-based bracket depth, or null when no bracket depth was collected.

---

### text

> `readonly` **text**: `string`

Defined in: [types.ts:133](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L133)

Source text without newline separators.
