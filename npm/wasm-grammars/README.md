# @comfit/wasm-grammars

Comfit grammar assets and queries for WebAssembly, with one import per language.

[![npm](https://img.shields.io/npm/v/@comfit/wasm-grammars.svg)](https://www.npmjs.com/package/@comfit/wasm-grammars)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [npm](https://www.npmjs.com/package/@comfit/wasm-grammars)

## Installation

```sh
npm install @comfit/wasm @comfit/wasm-grammars
```

## Usage

First [initialize the WASM runtimes](../wasm/README.md#usage), then load a language. Serve `tree-sitter-rust.wasm` at the URL used below.

```ts
import { createHighlighter } from "@comfit/wasm";
import rust from "@comfit/wasm-grammars/rust";

const highlighter = await createHighlighter({
  languages: [{ ...rust, wasm: "/assets/tree-sitter-rust.wasm" }],
});

try {
  const html = highlighter.renderHtml("fn main() {}", "rust");
  console.log(html);
} finally {
  highlighter.dispose();
}
```

See the [Languages guide](https://comfit.pinanek.me/docs/guides/languages) and [embedded-language guide](https://comfit.pinanek.me/docs/guides/embedded-languages) for asset loading and language selection.

## License

Licensed under the [MIT License](LICENSE).

Bundled grammar licenses and query attribution are included in
[THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt).
