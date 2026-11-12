# @comfit/node

Comfit bindings for Node.js.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [npm](https://www.npmjs.com/package/@comfit/node)

## Installation

```sh
npm install @comfit/node
```

## Usage

```js
import { renderHtml, RenderHtmlConfig, RenderHtmlStyle } from "@comfit/node";

const config = new RenderHtmlConfig();
config.style = RenderHtmlStyle.Inline;

const html = renderHtml(
  "fn main() {}",
  "rust",
  "catppuccin-mocha",
  config,
);
```

See the [Node.js guide](https://comfit.pinanek.me/docs/getting-started/node) for tokens, ANSI output, document sessions, and worker threads.

## License

Licensed under the [MIT License](LICENSE).

Bundled parser licenses and query attribution are included in [THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt).
