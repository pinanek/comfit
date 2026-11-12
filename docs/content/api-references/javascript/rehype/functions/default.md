[@comfit/rehype](../index.md) / default

# Function: default()

> **default**(`options?`): `Transformer`\<`Root`\>

Defined in: [index.ts:67](https://github.com/pinanek/comfit/blob/cce2d25a263a457dfebafc1502356a1de9f3952f/npm/rehype/src/index.ts#L67)

Highlight text-only pre/code blocks, preserving their attributes and source text.

Inline code, blocks with child markup, and unlabelled blocks without a default
language remain unchanged. Unknown languages are skipped unless configured as
errors. Rendering failures and invalid line metadata are reported through the VFile.
Calls on the same Node worker share a synchronous highlighting engine.

## Parameters

### options?

[`Options`](../interfaces/Options.md) = `{}`

## Returns

`Transformer`\<`Root`\>
