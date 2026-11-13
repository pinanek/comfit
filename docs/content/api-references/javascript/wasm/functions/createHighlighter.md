[@comfit/wasm](../index.md) / createHighlighter

# Function: createHighlighter()

> **createHighlighter**(`config`): `Promise`\<[`Highlighter`](../interfaces/Highlighter.md)\>

Defined in: [index.ts:98](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/index.ts#L98)

Create a highlighter and load its grammars and queries.

Await [initialize](initialize.md) first. If creation fails, partially loaded resources are released.
Call [Highlighter.dispose](../interfaces/Highlighter.md#dispose) when the highlighter is no longer needed.

## Parameters

### config

[`CreateHighlighterOptions`](../interfaces/CreateHighlighterOptions.md)

## Returns

`Promise`\<[`Highlighter`](../interfaces/Highlighter.md)\>

## Throws

Rejects before initialization, for failed asset loads, or for invalid language definitions.
