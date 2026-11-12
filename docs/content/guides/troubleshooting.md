---
title: "Troubleshooting"
description: "Resolve common Comfit installation, grammar loading, highlighting, and rendering problems."
---

# Troubleshooting

## HTML has no colors

The libraries default to class output, which needs a stylesheet. Check the rendering mode before changing the theme:

- For inline output, supply a theme and select inline mode.
- For class output, load an `@comfit/css` stylesheet and set its matching `preClass`. For example, `catppuccin-mocha.css` expects `preClass: "comfit-catppuccin-mocha"`.
- If you set a custom `classPrefix` or outer tag, make sure your stylesheet's selectors match the generated HTML.

See [HTML and CSS](html.md) for each mode and its CSS requirements.

Fonts, padding, and visible line numbers are application layout, not theme output.

## A language is unknown

Language names are registry IDs or registered aliases, not arbitrary display labels. List them with `languages()` in Node.js, `comfit.languages()` in Python, or `comfit ansi --list-languages` in the CLI.

In Rust, enable the relevant Cargo feature before creating the highlighter. In the browser, pass its descriptor to `createHighlighter`. An alias does not load another parser.

The playground lists only the languages it loads, not every grammar Comfit can bundle.

## Markdown fences are not highlighted

Load the fenced language as well as Markdown. In the browser, load `markdown-inline` too if you want inline Markdown styles.

An unknown injected language is skipped rather than reported as an unknown root language. A query asking for CSS, SQL, or another embedded language does not guarantee that its grammar is bundled. See [Embedded languages](embedded-languages.md).

## Browser WASM fails to load

1. Check the browser's network panel for `comfit_bg.wasm`, `web-tree-sitter.wasm`, and every selected parser WASM file. Each must load successfully.
2. Serve the application over HTTP, not a `file:` URL. Check that asset paths account for your deployment's base path and that cross-origin hosts allow the requests through CORS.
3. Await initialization of both runtimes before creating a highlighter.

The [browser quickstart](../getting-started/browser.md) uses bundler-generated URLs to avoid copying assets by hand. For source builds, use the pinned CLI versions in [Build from source](../development/build-from-source.md).

## Brackets or selected lines have no color

Collecting a bracket depth or selecting a line does not supply its color. Add a rainbow palette and a line-highlight style to the theme, or define the corresponding CSS classes or variables.

The bundled Catppuccin Mocha theme has no rainbow palette or line-highlight background. See [Themes](themes.md).

## The native addon cannot load

A Node.js native addon must match your operating system and CPU architecture. A binary built on one platform does not cover the other targets. Build `@comfit/node` on your platform when using a checkout.

Python also uses a native extension. If installation needs to compile it, provide Rust, C/C++ build tools, and a supported Python version. See [Build from source](../development/build-from-source.md).

## The page pauses while highlighting

Browser and Node.js highlighting calls are synchronous. Move large workloads to a Web Worker or Node worker rather than blocking the UI or server event loop.

Reuse a highlighter instead of recompiling queries for every snippet. For repeated edits, use a [document session](documents.md).

## ANSI output loses color in a pipe

The CLI's automatic color policy disables styles for redirected output, a nonempty `NO_COLOR`, or `TERM=dumb`. Use `--color always` when you want ANSI styles in a pipe or file.

Library ANSI renderers do not apply that policy for you. Your application decides whether to emit color.
