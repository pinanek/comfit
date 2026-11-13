[@comfit/wasm](../index.md) / initialize

# Function: initialize()

> **initialize**(`options`): `Promise`\<`void`\>

Defined in: [index.ts:68](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/index.ts#L68)

Initialize Comfit and web-tree-sitter from WASM URLs or bytes.

Concurrent calls share the first attempt. Successful initialization is reused.
A failed attempt can be retried with new sources.

## Parameters

### options

[`InitializeOptions`](../interfaces/InitializeOptions.md)

## Returns

`Promise`\<`void`\>

## Throws

Rejects if a source cannot be fetched or either runtime fails to initialize.
