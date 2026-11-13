---
title: "markdown-it"
description: "Install and configure @comfit/markdown-it to highlight fenced code blocks in markdown-it pipelines."
---

# markdown-it

Use `@comfit/markdown-it` to highlight fenced code blocks when rendering Markdown with markdown-it. It runs on Node.js 22 or later and bundles languages and themes. For rehype or MDX pipelines, use [the rehype plugin](markdown.md) instead.

This differs from [highlighting Markdown source](embedded-languages.md), which preserves headings, fence markers, and other Markdown syntax as source text.

## Install

<CodeTabs label="Package manager">
<CodeTab label="pnpm">

```sh
pnpm add @comfit/markdown-it markdown-it
```

</CodeTab>
<CodeTab label="npm">

```sh
npm install @comfit/markdown-it markdown-it
```

</CodeTab>
<CodeTab label="bun">

```sh
bun add @comfit/markdown-it markdown-it
```

</CodeTab>
</CodeTabs>

## Render Markdown

Save this as `render-markdown.mjs`:

````javascript title="render-markdown.mjs"
import markdownItComfit from "@comfit/markdown-it";
import MarkdownIt from "markdown-it";

const md = new MarkdownIt().use(markdownItComfit, {
  theme: "catppuccin-mocha",
});
console.log(md.render("```rust\nfn main() {}\n```\n"));
````

Run `node render-markdown.mjs`. The result contains highlighted fenced code blocks with inline colors. For class-based output, use a [theme stylesheet](html.md#classes).

The plugin preserves token attributes and language classes. Inline and indented code are unchanged.

## Use with markdown-it-async

If your pipeline needs asynchronous highlighting callbacks, consider [markdown-it-async](https://github.com/antfu/markdown-it-async). It wraps markdown-it and supports the same Comfit plugin.

Install `markdown-it-async` alongside `@comfit/markdown-it`:

```sh
pnpm add @comfit/markdown-it markdown-it-async
```

````javascript
import markdownItComfit from "@comfit/markdown-it";
import MarkdownItAsync from "markdown-it-async";

const md = MarkdownItAsync().use(markdownItComfit, {
  theme: "catppuccin-mocha",
});
console.log(await md.renderAsync("```rust\nfn main() {}\n```\n"));
````

Use `renderAsync` instead of `render`. Comfit's highlighting remains synchronous; this wrapper does not move it off the calling thread.

## Emphasize lines

Use `highlightedLines` to select one-based line numbers in every fence:

```javascript
const md = new MarkdownIt().use(markdownItComfit, {
  theme: "catppuccin-mocha",
  highlightedLines: [2, 4, 5, 6],
});
```

Fence metadata after the language name is ignored, including ranges such as `{2,4-6}`. Unlike the rehype plugin, markdown-it does not read line selections from fence metadata.

Selected lines receive a `line-highlight` class. To make the emphasis visible, add a CSS rule or a [theme line-highlight style](themes.md#emphasize-lines).

## Decide how to handle unknown languages

Unknown languages use the previous fence renderer by default. Use `unknownLanguage: "error"` to catch misspelled fence languages during a build. Use `defaultLanguage` if unlabelled fences should have a known language.

The plugin highlights source code, but does not render Markdown diagrams or sanitize the rest of your document.

See the [markdown-it API reference](../api-references/javascript/markdown-it/index.md) for aliases, named themes, and all options.
