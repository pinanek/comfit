[@comfit/rehype](../index.md) / Options

# Interface: Options

Defined in: [index.ts:26](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L26)

Rendering, language selection, and line emphasis for code blocks.

## Properties

### aliases?

> `optional` **aliases?**: `Readonly`\<`Record`\<`string`, `string`\>\>

Defined in: [index.ts:32](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L32)

Additional aliases, resolved once before checking built-in IDs and aliases.

---

### classPrefix?

> `optional` **classPrefix?**: `string`

Defined in: [index.ts:46](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L46)

Prefix for generated token, rainbow, and line classes. Defaults to an empty string.

---

### defaultLanguage?

> `optional` **defaultLanguage?**: `string`

Defined in: [index.ts:30](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L30)

Language for blocks without a language class. Otherwise leave them unchanged.

---

### defaultTheme?

> `optional` **defaultTheme?**: `string`

Defined in: [index.ts:38](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L38)

Default name in `themes`. Requires `themes`.

---

### highlightedLines?

> `optional` **highlightedLines?**: readonly `number`[]

Defined in: [index.ts:54](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L54)

One-based source lines to emphasize in every block. Defaults to an empty list.

---

### lineTag?

> `optional` **lineTag?**: `string`

Defined in: [index.ts:50](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L50)

Line tag name. Defaults to "span".

---

### metaHighlight?

> `optional` **metaHighlight?**: `boolean`

Defined in: [index.ts:56](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L56)

Read line ranges such as `{2,4-6}` from code.data.meta. Defaults to `true`.

---

### preClass?

> `optional` **preClass?**: `string`

Defined in: [index.ts:52](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L52)

Classes appended to the existing outer element's classes.

---

### preTag?

> `optional` **preTag?**: `string`

Defined in: [index.ts:48](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L48)

Outer tag name. Defaults to "pre".

---

### rainbow?

> `optional` **rainbow?**: `boolean`

Defined in: [index.ts:42](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L42)

Collect rainbow bracket depths. Defaults to `true`.

---

### rainbowColors?

> `optional` **rainbowColors?**: `number`

Defined in: [index.ts:44](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L44)

Rainbow class wrapping count. Zero preserves depth without a theme palette. Defaults to zero.

---

### style?

> `optional` **style?**: `"class"` \| `"inline"` \| `"variables"`

Defined in: [index.ts:40](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L40)

Defaults to `"inline"` with themes, otherwise `"class"`. Inline output requires a theme.

---

### theme?

> `optional` **theme?**: `string`

Defined in: [index.ts:34](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L34)

Bundled theme ID. Cannot be combined with `themes`.

---

### themes?

> `optional` **themes?**: `Readonly`\<`Record`\<`string`, `string`\>\>

Defined in: [index.ts:36](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L36)

Named bundled themes for inline output. The first name is the default unless selected.

---

### unknownLanguage?

> `optional` **unknownLanguage?**: `"skip"` \| `"error"`

Defined in: [index.ts:28](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/rehype/src/index.ts#L28)

Skip unknown languages or fail the file. Defaults to `"skip"`.
