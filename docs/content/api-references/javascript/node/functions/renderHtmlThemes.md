[@comfit/node](../index.md) / renderHtmlThemes

# Function: renderHtmlThemes()

> **renderHtmlThemes**(`source`, `language`, `themes`, `defaultTheme`, `config?`): `string`

Defined in: index.d.ts:176

Render inline HTML with a default theme and named alternative themes.

Alternatives use `--comfit-<name>` CSS properties. Caller CSS activates them.
Omitted configuration defaults to inline style.

# Errors

Returns `InvalidArg` for invalid themes, names, default selection, non-inline style,
languages, source ranges, or numeric options. Backend failures and excessive
injection nesting return `GenericFailure`.

## Parameters

### source

`string`

### language

`string`

### themes

[`NamedTheme`](../interfaces/NamedTheme.md)[]

### defaultTheme

`string`

### config?

[`RenderHtmlConfig`](../classes/RenderHtmlConfig.md) \| `null`

## Returns

`string`
