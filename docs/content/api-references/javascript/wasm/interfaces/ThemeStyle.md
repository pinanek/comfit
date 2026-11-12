[@comfit/wasm](../index.md) / ThemeStyle

# Interface: ThemeStyle

Defined in: [types.ts:50](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L50)

Optional colors, font modifiers, and underline settings for one capture.

## Properties

### bg?

> `readonly` `optional` **bg?**: [`ThemeColor`](../type-aliases/ThemeColor.md)

Defined in: [types.ts:54](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L54)

Background color. Inherits the parent value if omitted.

---

### fg?

> `readonly` `optional` **fg?**: [`ThemeColor`](../type-aliases/ThemeColor.md)

Defined in: [types.ts:52](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L52)

Foreground color. Inherits the parent value if omitted.

---

### modifiers?

> `readonly` `optional` **modifiers?**: readonly [`ThemeModifier`](../type-aliases/ThemeModifier.md)[]

Defined in: [types.ts:56](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L56)

An omitted or empty list clears inherited modifiers on matching captures.

---

### underline?

> `readonly` `optional` **underline?**: `object`

Defined in: [types.ts:58](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/wasm/js/types.ts#L58)

Underline settings. Inherits the parent settings if omitted.

#### color?

> `readonly` `optional` **color?**: [`ThemeColor`](../type-aliases/ThemeColor.md)

Underline color. Uses the rendering default if omitted.

#### style?

> `readonly` `optional` **style?**: `"line"` \| `"curl"` \| `"dashed"` \| `"dotted"` \| `"double_line"`

Underline shape. Uses a straight line if omitted.
