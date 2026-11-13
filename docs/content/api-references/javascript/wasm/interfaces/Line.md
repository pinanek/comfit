[@comfit/wasm](../index.md) / Line

# Interface: Line

Defined in: [types.ts:141](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L141)

A source line without its newline separator, including empty source lines.

## Properties

### highlighted

> `readonly` **highlighted**: `boolean`

Defined in: [types.ts:145](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L145)

Whether this line is selected for emphasis.

---

### number

> `readonly` **number**: `number`

Defined in: [types.ts:143](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L143)

One-based source line number.

---

### tokens

> `readonly` **tokens**: readonly [`Token`](Token.md)[]

Defined in: [types.ts:147](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L147)

Tokens in source order. Token boundaries may differ from syntax node boundaries.
