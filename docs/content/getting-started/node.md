---
title: "Node.js"
description: "Highlight code and render HTML or ANSI output with @comfit/node."
---

# Node.js

Use `@comfit/node` to highlight code on a server or in a build script. It provides synchronous functions with bundled languages and themes, and supports ESM on Node.js 22 or later.

## Install

<CodeTabs label="Package manager">
<CodeTab label="pnpm">

```sh
pnpm add @comfit/node
```

</CodeTab>
<CodeTab label="npm">

```sh
npm install @comfit/node
```

</CodeTab>
<CodeTab label="bun">

```sh
bun add @comfit/node
```

</CodeTab>
</CodeTabs>

The package uses a platform-specific native addon. If you are working from a checkout, [build the addon first](../development/build-from-source.md#nodejs-and-rehype).

## Render HTML

Save this as `highlight.mjs`:

```javascript title="highlight.mjs"
import { renderHtml, RenderHtmlConfig, RenderHtmlStyle } from "@comfit/node";

const config = new RenderHtmlConfig();
config.style = RenderHtmlStyle.Inline;

const html = renderHtml(
  "const answer = 42;",
  "javascript",
  "catppuccin-mocha",
  config,
);

console.log(html);
```

Run `node highlight.mjs` to print an HTML fragment with inline colors. Your page's CSS controls its font, padding, and layout.

For HTML rendering, pass a `RenderHtmlConfig` instance, not a plain object. The default style is classes, which requires a [theme stylesheet](../guides/html.md#classes). Selecting `Inline` embeds the colors in the fragment.

## Find languages and themes

```javascript
import { containsLanguage, languages, themes } from "@comfit/node";

console.log(languages());
console.log(themes());
console.log(containsLanguage("rs"));
```

Language IDs and aliases are accepted, such as `rust` or `rs`. An unknown root language or theme throws an error.

## Choose another output

`highlight(source, language)` returns [lines and tokens](../guides/tokens.md). `renderAnsi(source, language, theme)` returns styled terminal text. For class-based HTML, use `renderHtmlClasses` with a [CSS theme](../guides/html.md#classes). For Markdown or MDX code blocks, use [rehype](../guides/markdown.md).

Calls reuse a cached highlighter and block the calling JavaScript thread. Use a Node worker for large inputs. For repeated edits to one source, use a [document session](../guides/documents.md).

See the [Node.js API reference](../api-references/javascript/node/index.md) for configuration fields and methods.
