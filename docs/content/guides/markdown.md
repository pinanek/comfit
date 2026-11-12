---
title: "Markdown and MDX"
description: "Install and configure @comfit/rehype to highlight code blocks in Markdown and MDX sites."
---

# Markdown and MDX

Use `@comfit/rehype` to highlight code blocks when rendering Markdown or MDX. It runs on Node.js 22 or later and bundles languages and themes.

This differs from [highlighting Markdown source](embedded-languages.md), which preserves headings, fence markers, and other Markdown syntax as source text.

The plugin returns elements in HAST, rehype's HTML syntax tree, rather than raw HTML nodes. Its output does not require `rehype-raw`.

## Install

The commands below install the plugin and packages for a standalone Markdown pipeline. If your framework already processes MDX, install only `@comfit/rehype` and continue with [Use with MDX](#use-with-mdx).

<CodeTabs label="Package manager">
<CodeTab label="pnpm">

```sh
pnpm add @comfit/rehype unified remark-parse remark-rehype rehype-stringify
```

</CodeTab>
<CodeTab label="npm">

```sh
npm install @comfit/rehype unified remark-parse remark-rehype rehype-stringify
```

</CodeTab>
<CodeTab label="bun">

```sh
bun add @comfit/rehype unified remark-parse remark-rehype rehype-stringify
```

</CodeTab>
</CodeTabs>

## Render Markdown

Save this as `render-markdown.mjs`:

````javascript title="render-markdown.mjs"
import rehypeComfit from "@comfit/rehype";
import rehypeStringify from "rehype-stringify";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";

const file = await unified()
  .use(remarkParse)
  .use(remarkRehype)
  .use(rehypeComfit, { theme: "catppuccin-mocha" })
  .use(rehypeStringify)
  .process("```rust\nfn main() {}\n```\n");

console.log(String(file));
````

Run `node render-markdown.mjs`. The result contains highlighted code blocks with inline colors.

Place Comfit after `remark-rehype` and before `rehype-stringify`. For class-based output, use a [theme stylesheet](html.md#classes).

## Use with markdown-it

For a markdown-it pipeline, install `@comfit/markdown-it` and `markdown-it`:

```sh
npm install @comfit/markdown-it markdown-it
```

````javascript
import markdownItComfit from "@comfit/markdown-it";
import MarkdownIt from "markdown-it";

const md = new MarkdownIt().use(markdownItComfit, {
  theme: "catppuccin-mocha",
});
console.log(md.render("```rust\nfn main() {}\n```\n"));
````

This plugin highlights fenced code blocks, preserves token attributes and language classes, and uses the previous fence renderer for unknown languages. Inline and indented code are unchanged. Fence metadata after the language name is ignored; use `highlightedLines` to select lines.

See the [markdown-it API reference](../api-references/javascript/markdown-it/index.md) for options.

## Use with MDX

Add Comfit to your framework's `rehypePlugins` list:

```javascript
import rehypeComfit from "@comfit/rehype";

const rehypePlugins = [
  [rehypeComfit, { theme: "catppuccin-mocha" }],
];
```

## Emphasize lines

Add one-based line numbers or inclusive ranges to fence metadata:

````markdown
```rust {2,4-6}
fn main() {
    let value = 1;

    if value > 0 {
        println!("{value}");
    }
}
```
````

This selects line 2 and lines 4 through 6 within the code block. Fence metadata selections combine with the plugin's `highlightedLines` option. Invalid numeric ranges fail the build.

Selected lines receive a `line-highlight` class. To make the emphasis visible, add a CSS rule or a [theme line-highlight style](themes.md#emphasize-lines).

## Decide how to handle unknown languages

Unknown languages are left unchanged by default. Use `unknownLanguage: "error"` to catch misspelled fence languages during a build. Use `defaultLanguage` if unlabelled code blocks should have a known language.

The plugin changes text-only block code. Inline code and blocks that already contain child markup are left alone. It highlights source code, but does not render Markdown diagrams or sanitize the rest of your document.

See the [rehype API reference](../api-references/javascript/rehype/index.md) for aliases, named themes, and all options.
