---
title: "Comfit documentation"
description: "Choose a Comfit package and turn source code into highlighted HTML, terminal output, or tokens."
---

# Comfit documentation

Comfit turns source code into highlighted HTML, terminal output, or tokens for your own renderer. It uses Tree-sitter to parse code and a shared highlighting engine across Rust, Node.js, Python, and WebAssembly.

This site uses Comfit to highlight its code examples through the [rehype plugin](guides/rehype.md). The [playground](/playground) runs Comfit in your browser so you can try your own code before installing a package.

## Choose your environment

| Where you need highlighting      | Start here                             |
| -------------------------------- | -------------------------------------- |
| A Node.js server or build script | [Node.js](getting-started/node.md)     |
| A browser application            | [Browser](getting-started/browser.md)  |
| A Rust application               | [Rust](getting-started/rust.md)        |
| A Python application             | [Python](getting-started/python.md)    |
| A terminal or shell script       | [Command line](getting-started/cli.md) |

The Node.js, Python, and CLI packages bundle languages and themes. Rust uses Cargo features to select them. In the browser, you load the grammars you need and supply CSS or theme data. Start with your environment's quickstart for installation and a first render.

## Packages

Version badges show the latest published version of each package.

| Package                                                                      | Version                                                                                                                                        | Purpose                                        |
| ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| [comfit](https://crates.io/crates/comfit)                                    | [![comfit on crates.io](https://img.shields.io/crates/v/comfit.svg)](https://crates.io/crates/comfit)                                          | Rust syntax highlighting library               |
| [comfit-cli](https://github.com/pinanek/comfit/releases/latest)              | [![comfit-cli on crates.io](https://img.shields.io/crates/v/comfit-cli.svg)](https://crates.io/crates/comfit-cli)                              | Command-line highlighting and theme conversion |
| [comfit-core](https://crates.io/crates/comfit-core)                          | [![comfit-core on crates.io](https://img.shields.io/crates/v/comfit-core.svg)](https://crates.io/crates/comfit-core)                           | Highlighting engine, themes, and renderers     |
| [comfit-grammars](https://crates.io/crates/comfit-grammars)                  | [![comfit-grammars on crates.io](https://img.shields.io/crates/v/comfit-grammars.svg)](https://crates.io/crates/comfit-grammars)               | Rust parsers and queries                       |
| [comfit-themes](https://crates.io/crates/comfit-themes)                      | [![comfit-themes on crates.io](https://img.shields.io/crates/v/comfit-themes.svg)](https://crates.io/crates/comfit-themes)                     | Bundled themes and VS Code theme conversion    |
| [@comfit/node](https://www.npmjs.com/package/@comfit/node)                   | [![@comfit/node on npm](https://img.shields.io/npm/v/@comfit/node.svg)](https://www.npmjs.com/package/@comfit/node)                            | Node.js bindings                               |
| [@comfit/markdown-it](https://www.npmjs.com/package/@comfit/markdown-it)     | [![@comfit/markdown-it on npm](https://img.shields.io/npm/v/@comfit/markdown-it.svg)](https://www.npmjs.com/package/@comfit/markdown-it)       | markdown-it fenced code highlighting           |
| [@comfit/rehype](https://www.npmjs.com/package/@comfit/rehype)               | [![@comfit/rehype on npm](https://img.shields.io/npm/v/@comfit/rehype.svg)](https://www.npmjs.com/package/@comfit/rehype)                      | rehype and MDX code block highlighting         |
| [@comfit/wasm](https://www.npmjs.com/package/@comfit/wasm)                   | [![@comfit/wasm on npm](https://img.shields.io/npm/v/@comfit/wasm.svg)](https://www.npmjs.com/package/@comfit/wasm)                            | WebAssembly bindings for browsers and Node.js  |
| [@comfit/wasm-grammars](https://www.npmjs.com/package/@comfit/wasm-grammars) | [![@comfit/wasm-grammars on npm](https://img.shields.io/npm/v/@comfit/wasm-grammars.svg)](https://www.npmjs.com/package/@comfit/wasm-grammars) | WebAssembly grammar assets and queries         |
| [@comfit/css](https://www.npmjs.com/package/@comfit/css)                     | [![@comfit/css on npm](https://img.shields.io/npm/v/@comfit/css.svg)](https://www.npmjs.com/package/@comfit/css)                               | Theme stylesheets for class-based HTML         |
| [comfit-hl](https://pypi.org/project/comfit-hl/)                             | [![comfit-hl on PyPI](https://img.shields.io/pypi/v/comfit-hl.svg)](https://pypi.org/project/comfit-hl/)                                       | Python bindings                                |

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

Use [rehype and MDX](guides/rehype.md) or [markdown-it](guides/markdown-it.md) to highlight fenced code blocks in your Markdown pipeline. These integrations highlight code blocks in rendered pages, rather than highlighting Markdown source itself.

## Look up an API

The API references list exported types, functions, and methods:

- JavaScript: [Node.js](api-references/javascript/node/index.md), [WebAssembly](api-references/javascript/wasm/index.md), [rehype](api-references/javascript/rehype/index.md), and [WASM grammars](api-references/javascript/wasm-grammars/index.md).
- Python: [comfit](api-references/python/index.md).
- Rust: [comfit](https://docs.rs/comfit/latest/comfit/), [comfit-core](https://docs.rs/comfit-core/latest/comfit_core/), [comfit-grammars](https://docs.rs/comfit-grammars/latest/comfit_grammars/), and [comfit-themes](https://docs.rs/comfit-themes/latest/comfit_themes/).

For setup problems, see [Troubleshooting](guides/troubleshooting.md). To work from this repository, follow [Build from source](development/build-from-source.md).
