[@comfit/wasm](../index.md) / HtmlOptions

# Interface: HtmlOptions

Defined in: [types.ts:81](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L81)

Element classes and rainbow wrapping shared by all HTML style modes.

## Extends

- [`HighlightOptions`](HighlightOptions.md)

## Properties

### classPrefix?

> `optional` **classPrefix?**: `string`

Defined in: [types.ts:89](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L89)

Prefix for token, rainbow, and line classes. Defaults to an empty string.

---

### highlightedLines?

> `optional` **highlightedLines?**: readonly `number`[]

Defined in: [types.ts:30](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L30)

One-based source lines to emphasize. Defaults to an empty list.

#### Inherited from

[`HighlightOptions`](HighlightOptions.md).[`highlightedLines`](HighlightOptions.md#highlightedlines)

---

### lineTag?

> `optional` **lineTag?**: `string`

Defined in: [types.ts:85](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L85)

Line tag name. Defaults to "span".

---

### preClass?

> `optional` **preClass?**: `string`

Defined in: [types.ts:87](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L87)

Optional class on the outer element, escaped as attribute text.

---

### preTag?

> `optional` **preTag?**: `string`

Defined in: [types.ts:83](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L83)

Outer tag name. Defaults to "pre".

---

### rainbow?

> `optional` **rainbow?**: `boolean`

Defined in: [types.ts:32](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L32)

Collect bracket depths. Defaults to true.

#### Inherited from

[`HighlightOptions`](HighlightOptions.md).[`rainbow`](HighlightOptions.md#rainbow)

---

### rainbowColors?

> `optional` **rainbowColors?**: `number`

Defined in: [types.ts:91](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L91)

Wrap class/variable depths at this count. Zero preserves depth. Inline themes use their palettes.
