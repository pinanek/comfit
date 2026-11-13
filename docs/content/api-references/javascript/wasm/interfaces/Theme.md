[@comfit/wasm](../index.md) / Theme

# Interface: Theme

Defined in: [types.ts:67](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L67)

Theme data supplied directly to the WASM engine for validation.

## Properties

### default?

> `readonly` `optional` **default?**: [`ThemeStyle`](ThemeStyle.md)

Defined in: [types.ts:71](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L71)

Base style inherited by tokens.

---

### lineHighlight?

> `readonly` `optional` **lineHighlight?**: [`ThemeStyle`](ThemeStyle.md)

Defined in: [types.ts:77](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L77)

Style applied to source lines selected for emphasis.

---

### name?

> `readonly` `optional` **name?**: `string`

Defined in: [types.ts:69](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L69)

Display name. Named-theme CSS properties use the keys in the theme map instead.

---

### rainbow?

> `readonly` `optional` **rainbow?**: readonly [`ThemeColor`](../type-aliases/ThemeColor.md)[]

Defined in: [types.ts:75](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L75)

Bracket colors wrapped by depth. An empty palette disables theme bracket colors.

---

### styles?

> `readonly` `optional` **styles?**: `Readonly`\<`Record`\<`string`, [`ThemeStyle`](ThemeStyle.md)\>\>

Defined in: [types.ts:73](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L73)

Capture styles keyed by dotted query capture names.
