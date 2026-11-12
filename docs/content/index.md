---
title: "Comfit documentation"
description: "Choose a Comfit package and turn source code into highlighted HTML, terminal output, or tokens."
---

# Comfit documentation

Comfit turns source code into highlighted HTML, terminal output, or tokens for your own renderer. It uses Tree-sitter to parse code and a shared highlighting engine across Rust, Node.js, Python, and WebAssembly.

You can [try your own code in the playground](/playground) before installing a package.

## Choose your environment

| Where you need highlighting      | Start here                             |
| -------------------------------- | -------------------------------------- |
| A Node.js server or build script | [Node.js](getting-started/node.md)     |
| A browser application            | [Browser](getting-started/browser.md)  |
| A Rust application               | [Rust](getting-started/rust.md)        |
| A Python application             | [Python](getting-started/python.md)    |
| A terminal or shell script       | [Command line](getting-started/cli.md) |

The Node.js, Python, and CLI packages bundle languages and themes. Rust uses Cargo features to select them. In the browser, you load the grammars you need and supply CSS or theme data. Start with your environment's quickstart for installation and a first render.

## How highlighting works

A grammar parses source code into a syntax tree. Queries select parts of that tree and assign capture names such as `keyword`, `string`, or `function.call`. A theme maps those names to colors and font styles.

Comfit can render the result as HTML or terminal text, or return lines and tokens for your own renderer. HTML output escapes the source text, but your application supplies fonts, spacing, scrolling, and visible line numbers.

For individual snippets, use the one-shot highlighting or rendering functions. For source that changes repeatedly, a document session retains syntax trees across edits. Its results can be rendered with different themes without highlighting again.

## Work with the output

- [HTML and CSS](guides/html.md): choose inline styles, classes, or CSS variables and add layout styles.
- [Themes](guides/themes.md): choose palettes, color brackets, and emphasize lines.
- [Lines and tokens](guides/tokens.md): inspect captures or build a custom renderer.

## Work with source code

- [Languages](guides/languages.md): find language IDs, aliases, parser assets, and injection targets.
- [Embedded languages](guides/embedded-languages.md): highlight source that contains another language, such as Markdown with Rust code fences.
- [Updating source](guides/documents.md): reuse syntax trees, keep result snapshots, and manage document resources.

## Integrate Markdown and MDX

Follow [Markdown and MDX](guides/markdown.md) to install the rehype plugin, connect it to your pipeline, and configure highlighted code blocks. Use this integration for code blocks in rendered pages, rather than for highlighting Markdown source itself.

## Look up an API

The API references list exported types, functions, and methods:

- JavaScript: [Node.js](api-references/javascript/node/index.md), [WebAssembly](api-references/javascript/wasm/index.md), [rehype](api-references/javascript/rehype/index.md), and [WASM grammars](api-references/javascript/wasm-grammars/index.md).
- Python: [comfit](api-references/python/index.md).
- Rust: [comfit](https://docs.rs/comfit/latest/comfit/), [comfit-core](https://docs.rs/comfit-core/latest/comfit_core/), [comfit-grammars](https://docs.rs/comfit-grammars/latest/comfit_grammars/), and [comfit-themes](https://docs.rs/comfit-themes/latest/comfit_themes/).

For setup problems, see [Troubleshooting](guides/troubleshooting.md). To work from this repository, follow [Build from source](development/build-from-source.md).
