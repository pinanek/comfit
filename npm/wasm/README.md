# @comfit/wasm

Comfit WebAssembly bindings for browsers and Node.js.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [npm](https://www.npmjs.com/package/@comfit/wasm)

## Installation

```sh
npm install @comfit/wasm @comfit/wasm-grammars
```

## Usage

Serve `comfit_bg.wasm` and `web-tree-sitter.wasm` from this package's `dist` directory, plus the selected grammar WASMs from `@comfit/wasm-grammars/dist`. The example uses `/assets/` URLs.

```ts
import { createHighlighter, initialize } from "@comfit/wasm";
import rust from "@comfit/wasm-grammars/rust";

await initialize({
  engine: "/assets/comfit_bg.wasm",
  treeSitter: "/assets/web-tree-sitter.wasm",
});

const highlighter = await createHighlighter({
  languages: [{ ...rust, wasm: "/assets/tree-sitter-rust.wasm" }],
});

try {
  const html = highlighter.renderHtml("fn main() {}", "rust");
} finally {
  highlighter.dispose();
}
```

See the [browser guide](https://comfit.pinanek.me/docs/getting-started/browser) for bundlers, asset hosting, themes, and document sessions. A [local HTML example](examples/local.html) is included.

## License

Licensed under the [MIT License](LICENSE).

Third-party licenses and attribution, including the adapted ast-grep bindings
and web-tree-sitter runtime, are included in
[THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt).
