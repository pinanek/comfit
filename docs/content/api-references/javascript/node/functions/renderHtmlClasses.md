[@comfit/node](../index.md) / renderHtmlClasses

# Function: renderHtmlClasses()

> **renderHtmlClasses**(`source`, `language`, `config?`): `string`

Defined in: index.d.ts:152

Render escaped HTML without a theme.

Class and variable styles use caller-provided CSS. Inline style falls back to classes.

# Errors

Returns `InvalidArg` for invalid languages, source ranges, or numeric options.
Backend failures and excessive injection nesting return `GenericFailure`.

## Parameters

### source

`string`

### language

`string`

### config?

[`RenderHtmlConfig`](../classes/RenderHtmlConfig.md) \| `null`

## Returns

`string`
