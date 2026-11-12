[@comfit/wasm](../index.md) / HighlightOptions

# Interface: HighlightOptions

Defined in: [types.ts:28](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L28)

Options for line emphasis and bracket-depth collection.

## Extended by

- [`HtmlOptions`](HtmlOptions.md)

## Properties

### highlightedLines?

> `optional` **highlightedLines?**: readonly `number`[]

Defined in: [types.ts:30](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L30)

One-based source lines to emphasize. Defaults to an empty list.

---

### rainbow?

> `optional` **rainbow?**: `boolean`

Defined in: [types.ts:32](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L32)

Collect bracket depths. Defaults to true.
