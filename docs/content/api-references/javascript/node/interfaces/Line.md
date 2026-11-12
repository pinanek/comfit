[@comfit/node](../index.md) / Line

# Interface: Line

Defined in: index.d.ts:100

One source line without its newline separator. Tokens remain in source order.

## Properties

### highlighted

> **highlighted**: `boolean`

Defined in: index.d.ts:104

Whether this line is selected for emphasis.

---

### number

> **number**: `number`

Defined in: index.d.ts:102

One-based source line number.

---

### tokens

> **tokens**: [`Token`](Token.md)[]

Defined in: index.d.ts:106

Tokens in source order. Token boundaries may differ from syntax node boundaries.
