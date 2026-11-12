[@comfit/node](../index.md) / Document

# Class: Document

Defined in: index.d.ts:13

An incremental source document. Call dispose() when it is no longer needed.

## Constructors

### Constructor

> **new Document**(): `Document`

#### Returns

`Document`

## Accessors

### source

#### Get Signature

> **get** **source**(): `string`

Defined in: index.d.ts:14

##### Returns

`string`

## Methods

### dispose()

> **dispose**(): `void`

Defined in: index.d.ts:20

Release retained trees and language references. Repeated calls are harmless.

#### Returns

`void`

---

### highlight()

> **highlight**(`config?`): [`HighlightResult`](HighlightResult.md)

Defined in: index.d.ts:18

Return an owned snapshot. Changes to line emphasis reuse cached highlighting.

#### Parameters

##### config?

[`HighlightConfig`](HighlightConfig.md) \| `null`

#### Returns

[`HighlightResult`](HighlightResult.md)

---

### update()

> **update**(`source`): `void`

Defined in: index.d.ts:16

Incrementally parse changed source. If the source is unchanged, do nothing.

#### Parameters

##### source

`string`

#### Returns

`void`
