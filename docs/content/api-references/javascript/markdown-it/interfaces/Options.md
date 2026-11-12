[@comfit/markdown-it](../index.md) / Options

# Interface: Options

Defined in: index.ts:14

Language selection, themes, and rendering options for fenced code blocks.

## Properties

### aliases?

> `optional` **aliases?**: `Readonly`\<`Record`\<`string`, `string`\>\>

Defined in: index.ts:20

Additional aliases, resolved once before checking built-in IDs and aliases.

---

### classPrefix?

> `optional` **classPrefix?**: `string`

Defined in: index.ts:34

Prefix for generated token, rainbow, and line classes. Defaults to an empty string.

---

### defaultLanguage?

> `optional` **defaultLanguage?**: `string`

Defined in: index.ts:18

Language for fences without a language name. Otherwise leave them unchanged.

---

### defaultTheme?

> `optional` **defaultTheme?**: `string`

Defined in: index.ts:26

Default name in `themes`. Requires `themes`.

---

### highlightedLines?

> `optional` **highlightedLines?**: readonly `number`[]

Defined in: index.ts:38

One-based source lines to emphasize in every fence. Defaults to an empty list.

---

### preClass?

> `optional` **preClass?**: `string`

Defined in: index.ts:36

Class on the outer pre element, escaped as attribute text.

---

### rainbow?

> `optional` **rainbow?**: `boolean`

Defined in: index.ts:30

Collect rainbow bracket depths. Defaults to `true`.

---

### rainbowColors?

> `optional` **rainbowColors?**: `number`

Defined in: index.ts:32

Rainbow class wrapping count. Zero preserves depth without a theme palette.

---

### style?

> `optional` **style?**: `"class"` \| `"inline"` \| `"variables"`

Defined in: index.ts:28

Defaults to `"inline"` with themes, otherwise `"class"`. Inline output requires a theme.

---

### theme?

> `optional` **theme?**: `string`

Defined in: index.ts:22

Bundled theme ID. Cannot be combined with `themes`.

---

### themes?

> `optional` **themes?**: `Readonly`\<`Record`\<`string`, `string`\>\>

Defined in: index.ts:24

Named bundled themes for inline output. The first name is the default unless selected.

---

### unknownLanguage?

> `optional` **unknownLanguage?**: `"skip"` \| `"error"`

Defined in: index.ts:16

Skip unknown languages or throw. Defaults to `"skip"`.
