[@comfit/wasm](../index.md) / HighlightResult

# Interface: HighlightResult

Defined in: [types.ts:157](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L157)

An owned snapshot, independent of document edits or registry disposal.

## Properties

### lineCount

> `readonly` **lineCount**: `number`

Defined in: [types.ts:159](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L159)

Number of source lines, without exporting tokens from WASM.

---

### lines

> `readonly` **lines**: readonly [`Line`](Line.md)[]

Defined in: [types.ts:161](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L161)

Exported on first access. Read before disposal. Cached lines remain usable afterward.

## Methods

### dispose()

> **dispose**(): `void`

Defined in: [types.ts:165](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L165)

Release the WASM snapshot. Previously accessed lines remain usable. Repeated calls are harmless.

#### Returns

`void`

---

### renderHtml()

> **renderHtml**(`options?`): `string`

Defined in: [types.ts:163](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L163)

Render without parsing. Rainbow depths are fixed by document.highlight().

#### Parameters

##### options?

[`RenderHtmlOptions`](../type-aliases/RenderHtmlOptions.md)

#### Returns

`string`
