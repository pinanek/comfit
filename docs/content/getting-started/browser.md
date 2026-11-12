---
title: "Browser"
description: "Load the Comfit WebAssembly engine, language grammars, and themes to highlight code in the browser."
---

# Browser

Use `@comfit/wasm` when highlighting needs to run in the browser. You initialize the engine and Tree-sitter runtime, then load the grammars your application needs.

The bundler example below uses Vite's `?url` imports to serve WASM files. With another build tool, serve those files as public assets and pass their URLs instead. For a page without a bundler, see [Use a CDN](#use-a-cdn).

## Install

<CodeTabs label="Package manager">
<CodeTab label="pnpm">

```sh
pnpm add @comfit/wasm @comfit/wasm-grammars @comfit/css
```

</CodeTab>
<CodeTab label="npm">

```sh
npm install @comfit/wasm @comfit/wasm-grammars @comfit/css
```

</CodeTab>
<CodeTab label="bun">

```sh
bun add @comfit/wasm @comfit/wasm-grammars @comfit/css
```

</CodeTab>
</CodeTabs>

`@comfit/wasm` supplies the engine, `@comfit/wasm-grammars` supplies parser assets and queries, and `@comfit/css` supplies the theme stylesheet used below.

## Highlight a snippet

Add a container to your page:

```html
<div id="code"></div>
```

In your client entry point:

```typescript title="main.ts"
import { createHighlighter, initialize } from "@comfit/wasm";
import rust from "@comfit/wasm-grammars/rust";
import rustWasm from "@comfit/wasm-grammars/tree-sitter-rust.wasm?url";
import engine from "@comfit/wasm/comfit_bg.wasm?url";
import treeSitter from "@comfit/wasm/web-tree-sitter.wasm?url";
import "@comfit/css/catppuccin-mocha.css";

await initialize({ engine, treeSitter });

const highlighter = await createHighlighter({
  languages: [{ ...rust, wasm: rustWasm }],
});

try {
  const html = highlighter.renderHtml("fn main() {}", "rust", {
    preClass: "comfit-catppuccin-mocha",
  });
  const output = document.querySelector("#code");
  if (output) output.innerHTML = html;
} finally {
  highlighter.dispose();
}
```

The container now holds a highlighted HTML fragment. The [CSS theme](../guides/html.md#classes) supplies token colors. Your application's CSS controls fonts and spacing.

## Keep the highlighter when you need it

The example disposes the highlighter after one render. In an application, create it once, reuse it, and dispose it when its component or worker is finished. For repeated edits to the same source, use a [document session](../guides/documents.md).

Always await `initialize` before calling `createHighlighter`. Successful initialization is shared across calls.

Initialization and grammar loading are asynchronous, but highlighting and rendering are synchronous. Use a Web Worker for large inputs to avoid blocking the UI.

## Load more languages

Pass additional language descriptors to `createHighlighter`. Each descriptor provides a language's ID, aliases, and queries, along with the location of its parser asset. See [Languages](../guides/languages.md#load-browser-languages) for loading details and the supported-language catalog.

For Markdown source, load both the block and inline grammars, plus the languages used in code fences. See [Embedded languages](../guides/embedded-languages.md).

For inline theme objects or CSS variables, see [HTML and CSS](../guides/html.md) and [Themes](../guides/themes.md). The [WebAssembly API reference](../api-references/javascript/wasm/index.md) lists all options.

## Use a CDN

You can load Comfit's JavaScript, WASM files, and theme CSS from a CDN such as [esm.sh](https://esm.sh) or [jsDelivr](https://www.jsdelivr.com), without a bundler or package installation. The example below uses esm.sh for all files.

Save this as `index.html`:

```html title="index.html"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Comfit CDN example</title>
    <link
      rel="stylesheet"
      href="https://esm.sh/@comfit/css@0.1.0/dist/catppuccin-mocha.css"
    >
  </head>
  <body>
    <div id="code"></div>

    <script type="module">
    import rust from "https://esm.sh/@comfit/wasm-grammars@0.1.0/rust";
    import {
      createHighlighter,
      initialize,
    } from "https://esm.sh/@comfit/wasm@0.1.0";

    await initialize({
      engine: "https://esm.sh/@comfit/wasm@0.1.0/dist/comfit_bg.wasm",
      treeSitter: "https://esm.sh/@comfit/wasm@0.1.0/dist/web-tree-sitter.wasm",
    });

    const highlighter = await createHighlighter({
      languages: [{
        ...rust,
        wasm:
          "https://esm.sh/@comfit/wasm-grammars@0.1.0/dist/tree-sitter-rust.wasm",
      }],
    });

    try {
      document.querySelector("#code").innerHTML = highlighter.renderHtml(
        "fn main() {}",
        "rust",
        { preClass: "comfit-catppuccin-mocha" },
      );
    } finally {
      highlighter.dispose();
    }
    </script>
  </body>
</html>
```

Serve the file over HTTP rather than opening a `file:` URL. For example, run `python3 -m http.server 8000` in its directory and open `http://localhost:8000`.

For jsDelivr, use its `+esm` endpoints for JavaScript imports and regular file URLs for WASM and CSS.

Keep the engine, grammar, and stylesheet versions pinned together. Override the descriptor's `wasm` URL explicitly, as above, rather than relying on an asset path rewritten by a module CDN. Comfit does not choose a CDN or fall back to one automatically.
