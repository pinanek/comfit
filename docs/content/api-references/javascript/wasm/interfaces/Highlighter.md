[@comfit/wasm](../index.md) / Highlighter

# Interface: Highlighter

Defined in: [types.ts:180](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L180)

A parser and language registry. Call dispose() when no longer needed.

## Methods

### createDocument()

> **createDocument**(`options`): [`Document`](Document.md)

Defined in: [types.ts:182](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L182)

Create an independent document sharing the currently loaded language queries.

#### Parameters

##### options

[`CreateDocumentOptions`](CreateDocumentOptions.md)

#### Returns

[`Document`](Document.md)

---

### dispose()

> **dispose**(): `void`

Defined in: [types.ts:195](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L195)

Release runtime resources. Repeated calls are harmless. Later rendering calls throw.

#### Returns

`void`

---

### highlight()

> **highlight**(`source`, `language`, `options?`): readonly [`Line`](Line.md)[]

Defined in: [types.ts:188](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L188)

Highlight source into owned lines and tokens using a registered ID or alias.
Empty sources produce one line. A trailing newline produces a final empty line.

#### Parameters

##### source

`string`

##### language

`string`

##### options?

[`HighlightOptions`](HighlightOptions.md)

#### Returns

readonly [`Line`](Line.md)[]

#### Throws

If the registry is disposed, options are invalid, or highlighting fails.

---

### renderHtml()

> **renderHtml**(`source`, `language`, `options?`): `string`

Defined in: [types.ts:193](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L193)

Render an escaped HTML fragment with capture classes, inline themes, or CSS variables.

#### Parameters

##### source

`string`

##### language

`string`

##### options?

[`RenderHtmlOptions`](../type-aliases/RenderHtmlOptions.md)

#### Returns

`string`

#### Throws

If the registry is disposed, options are invalid, or highlighting fails.
