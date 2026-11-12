[@comfit/node](../index.md) / RenderHtmlConfig

# Class: RenderHtmlConfig

Defined in: index.d.ts:45

Highlighting and HTML options. Defaults to capture classes and bracket-depth collection.

## Constructors

### Constructor

> **new RenderHtmlConfig**(): `RenderHtmlConfig`

Defined in: index.d.ts:63

Create HTML options with capture classes and bracket-depth collection enabled.

#### Returns

`RenderHtmlConfig`

## Properties

### classPrefix

> **classPrefix**: `string`

Defined in: index.d.ts:57

Prefix for token, rainbow, and line classes. Defaults to an empty string.

---

### highlightedLines

> **highlightedLines**: `number`[]

Defined in: index.d.ts:47

One-based source lines to emphasize. Defaults to an empty list.

---

### lineTag

> **lineTag**: `string`

Defined in: index.d.ts:53

Line tag name. Defaults to `span`.

---

### preClass?

> `optional` **preClass?**: `string`

Defined in: index.d.ts:55

Optional class on the outer element, escaped as attribute text.

---

### preTag

> **preTag**: `string`

Defined in: index.d.ts:51

Outer tag name. Defaults to `pre`.

---

### rainbow

> **rainbow**: `boolean`

Defined in: index.d.ts:49

Collect bracket depths. Defaults to `true`.

---

### rainbowColors

> **rainbowColors**: `number`

Defined in: index.d.ts:59

Rainbow wrapping count. Zero preserves depth when no theme palette applies.

---

### style

> **style**: [`RenderHtmlStyle`](../enumerations/RenderHtmlStyle.md)

Defined in: index.d.ts:61

How token styles are rendered. Defaults to [`RenderHtmlStyle::Class`].
