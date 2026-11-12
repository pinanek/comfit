[@comfit/wasm](../index.md) / Document

# Interface: Document

Defined in: [types.ts:169](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L169)

A source document that keeps its syntax trees and cached highlighting.

## Properties

### source

> `readonly` **source**: `string`

Defined in: [types.ts:170](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L170)

## Methods

### dispose()

> **dispose**(): `void`

Defined in: [types.ts:176](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L176)

Release the document. Existing snapshots remain usable. Repeated calls are harmless.

#### Returns

`void`

---

### highlight()

> **highlight**(`options?`): [`HighlightResult`](HighlightResult.md)

Defined in: [types.ts:174](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L174)

Return an independent snapshot. Dispose each snapshot when no longer needed.

#### Parameters

##### options?

[`HighlightOptions`](HighlightOptions.md)

#### Returns

[`HighlightResult`](HighlightResult.md)

---

### update()

> **update**(`source`): `void`

Defined in: [types.ts:172](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L172)

Incrementally parse changed source. If the source is unchanged, do nothing.

#### Parameters

##### source

`string`

#### Returns

`void`
