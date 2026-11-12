[@comfit/node](../index.md) / HighlightResult

# Class: HighlightResult

Defined in: index.d.ts:34

An owned snapshot that remains usable after document edits or disposal.

## Constructors

### Constructor

> **new HighlightResult**(): `HighlightResult`

#### Returns

`HighlightResult`

## Accessors

### lines

#### Get Signature

> **get** **lines**(): [`Line`](../interfaces/Line.md)[]

Defined in: index.d.ts:36

Copy the snapshot's lines and tokens into JavaScript-owned values.

##### Returns

[`Line`](../interfaces/Line.md)[]

## Methods

### renderAnsi()

> **renderAnsi**(`theme`): `string`

Defined in: index.d.ts:41

#### Parameters

##### theme

`string`

#### Returns

`string`

---

### renderHtml()

> **renderHtml**(`theme`, `config?`): `string`

Defined in: index.d.ts:38

Render a snapshot without parsing. Bracket depths are set by highlight().

#### Parameters

##### theme

`string`

##### config?

[`RenderHtmlConfig`](RenderHtmlConfig.md) \| `null`

#### Returns

`string`

---

### renderHtmlClasses()

> **renderHtmlClasses**(`config?`): `string`

Defined in: index.d.ts:39

#### Parameters

##### config?

[`RenderHtmlConfig`](RenderHtmlConfig.md) \| `null`

#### Returns

`string`

---

### renderHtmlThemes()

> **renderHtmlThemes**(`themes`, `defaultTheme`, `config?`): `string`

Defined in: index.d.ts:40

#### Parameters

##### themes

[`NamedTheme`](../interfaces/NamedTheme.md)[]

##### defaultTheme

`string`

##### config?

[`RenderHtmlConfig`](RenderHtmlConfig.md) \| `null`

#### Returns

`string`
