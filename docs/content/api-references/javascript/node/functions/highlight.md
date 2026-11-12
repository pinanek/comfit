[@comfit/node](../index.md) / highlight

# Function: highlight()

> **highlight**(`source`, `language`, `config?`): [`Line`](../interfaces/Line.md)[]

Defined in: index.d.ts:94

Highlight source into owned lines and tokens.

Calls are synchronous. Use a Node worker for large inputs. Results remain usable after later calls.

# Errors

Returns `InvalidArg` for unknown languages, invalid ranges, or invalid numeric options.
Backend failures, excessive injection nesting, and output-index overflows return `GenericFailure`.

## Parameters

### source

`string`

### language

`string`

### config?

[`HighlightConfig`](../classes/HighlightConfig.md) \| `null`

## Returns

[`Line`](../interfaces/Line.md)[]
