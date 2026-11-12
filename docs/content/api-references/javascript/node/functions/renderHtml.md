[@comfit/node](../index.md) / renderHtml

# Function: renderHtml()

> **renderHtml**(`source`, `language`, `theme`, `config?`): `string`

Defined in: index.d.ts:140

Render escaped HTML with a bundled theme.

Class styles need a stylesheet. Variable styles need caller-provided CSS variables.

# Errors

Returns `InvalidArg` for invalid themes, languages, source ranges, or numeric options.
Backend failures and excessive injection nesting return `GenericFailure`.

## Parameters

### source

`string`

### language

`string`

### theme

`string`

### config?

[`RenderHtmlConfig`](../classes/RenderHtmlConfig.md) \| `null`

## Returns

`string`
