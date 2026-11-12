[@comfit/node](../index.md) / renderAnsi

# Function: renderAnsi()

> **renderAnsi**(`source`, `language`, `theme`, `config?`): `string`

Defined in: index.d.ts:128

Render ANSI-styled source with a bundled theme.

The core renderer escapes controls other than tabs and newlines. It does not add
CLI gutters, wrapping, padding, or color detection.

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

[`HighlightConfig`](../classes/HighlightConfig.md) \| `null`

## Returns

`string`
