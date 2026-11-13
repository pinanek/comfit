[@comfit/wasm](../index.md) / RenderHtmlOptions

# Type Alias: RenderHtmlOptions

> **RenderHtmlOptions** = [`HtmlOptions`](../interfaces/HtmlOptions.md) & (\{ `defaultTheme?`: `never`; `style?`: `"class"` \| `"variables"`; `theme?`: `never`; `themes?`: `never`; \} \| \{ `defaultTheme?`: `never`; `style`: `"inline"`; `theme`: [`Theme`](../interfaces/Theme.md); `themes?`: `never`; \} \| \{ `defaultTheme?`: `string`; `style`: `"inline"`; `theme?`: `never`; `themes`: `Readonly`\<`Record`\<`string`, [`Theme`](../interfaces/Theme.md)\>\>; \})

Defined in: [types.ts:95](https://github.com/pinanek/comfit/blob/9b5f953f31c4b250bc9e9a1adca09a2f1304f4bd/npm/wasm/js/types.ts#L95)

HTML style mode and its permitted single-theme or named-theme inputs.
