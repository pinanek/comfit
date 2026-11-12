[@comfit/wasm](../index.md) / createHighlighter

# Function: createHighlighter()

> **createHighlighter**(`options`): `Promise`\<[`Highlighter`](../interfaces/Highlighter.md)\>

Defined in: [index.ts:98](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/index.ts#L98)

Create a highlighter and load its grammars and queries.

Await [initialize](initialize.md) first. If creation fails, partially loaded resources are released.
Call [Highlighter.dispose](../interfaces/Highlighter.md#dispose) when the highlighter is no longer needed.

## Parameters

### options

[`CreateHighlighterOptions`](../interfaces/CreateHighlighterOptions.md)

## Returns

`Promise`\<[`Highlighter`](../interfaces/Highlighter.md)\>

## Throws

Rejects before initialization, for failed asset loads, or for invalid language definitions.
