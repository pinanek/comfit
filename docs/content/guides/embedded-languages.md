---
title: "Embedded languages"
description: "Highlight embedded languages with Comfit, including source code inside Markdown and other nested formats."
---

# Embedded languages

Comfit can highlight one language inside another, such as Rust inside a Markdown fence or JavaScript inside an HTML script block. These embedded regions are called injections. Tree-sitter injection queries identify each region and its language, then Comfit highlights it with a loaded grammar.

This guide covers highlighting the source itself. To turn Markdown into a web page with highlighted code blocks, use [rehype and MDX](rehype.md) or [markdown-it](markdown-it.md).

## Load the inner language

The outer grammar is not enough. A Rust fence inside Markdown needs Markdown and Rust, plus Markdown's inline grammar for bold text, links, and inline code.

Node.js, Python, and the CLI bundle these grammars. In Rust, enable the languages you need:

```sh
cargo add comfit --features markdown,rust,javascript,render-html
```

The `markdown` Cargo feature also enables `markdown_inline`.

## Understand missing languages

An unloaded root language is an error. An unloaded injected language is skipped, leaving the outer language's styling in place. A fence label or alias cannot load a parser by itself.

The bundled languages include the fixed injection targets used by the bundled queries: Regex, Printf, documentation grammars, template languages, and nested dependencies such as POD and DTD. See [Find a language](languages.md#find-a-language) for language-listing examples by API.

In Rust and browsers, load the target grammars you need alongside the host language. Dynamically selected languages, such as Markdown fence labels, may still refer to an unloaded language.

## Register browser grammars

With Vite, import parser URLs and pass the descriptors together. This assumes you have [initialized the browser runtimes](../getting-started/browser.md):

```typescript
import { createHighlighter } from "@comfit/wasm";
import markdown from "@comfit/wasm-grammars/markdown";
import markdownInline from "@comfit/wasm-grammars/markdown-inline";
import rust from "@comfit/wasm-grammars/rust";
import markdownWasm from "@comfit/wasm-grammars/tree-sitter-markdown.wasm?url";
import markdownInlineWasm from "@comfit/wasm-grammars/tree-sitter-markdown_inline.wasm?url";
import rustWasm from "@comfit/wasm-grammars/tree-sitter-rust.wasm?url";

const highlighter = await createHighlighter({
  languages: [
    { ...markdown, wasm: markdownWasm },
    { ...markdownInline, wasm: markdownInlineWasm },
    { ...rust, wasm: rustWasm },
  ],
});
```

Use `"markdown"` as the root language when rendering the whole document. Dispose the highlighter when finished.

## Check a fenced block

````markdown
# Example

Here is **bold text** and `inline code`.

```rust
fn main() {
    let message = "Hello, Comfit!";
    println!("{message}");
}
```
````

Comfit highlights this as Markdown source, including the fence markers. It does not convert the document into headings and paragraphs. For rendered Markdown with highlighted code blocks, use the [rehype plugin](rehype.md).

## HTML scripts and Bash heredocs

HTML queries inject JavaScript into ordinary `<script>` blocks and `type="module"` blocks. `text/*` and `application/*` MIME types select their subtype, such as `javascript` or `typescript`. Load the corresponding grammar alongside HTML.

Style blocks request the bundled CSS grammar.

Bash heredoc delimiters can also select an injected language. Use its registered ID or alias, for example lowercase `html`:

```bash
cat <<html
<article>
  <h1>Hello, Comfit!</h1>
</article>
html
```

Load both Bash and HTML to highlight this example. The browser playground loads both, along with JavaScript, TypeScript, and Python for nested content.

## Write injection queries

Injection queries use `@injection.content` for the source range and either `@injection.language` or an `injection.language` property for the target language.

By default, Comfit excludes named child nodes but retains unnamed punctuation, matching Neovim. This preserves Rust quotes and brackets while excluding Markdown continuation markers such as `>` in a blockquote. `injection.include-children` includes all children.

Combined injections preserve source coordinates rather than joining text. Nesting is limited to 32 injected layers. For custom grammar registration, see [Highlighter::load](https://docs.rs/comfit/latest/comfit/struct.Highlighter.html#method.load) and the browser [Language descriptor](../api-references/javascript/wasm/interfaces/Language.md).
