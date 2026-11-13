[@comfit/markdown-it](../index.md) / default

# Function: default()

> **default**(`md`, `options?`): `void`

Defined in: [index.ts:48](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/markdown-it/src/index.ts#L48)

Highlight fenced code blocks while preserving markdown-it's language classes and token attributes.

Inline code, indented code, and unlabelled fences without a default language are unchanged.
Unknown languages use the previous fence renderer unless configured as errors.
Calls on the same Node worker share a synchronous highlighting engine.

## Parameters

### md

`MarkdownIt`

### options?

[`Options`](../interfaces/Options.md) = `{}`

## Returns

`void`
