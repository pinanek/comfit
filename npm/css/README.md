# @comfit/css

Comfit theme stylesheets for class-based HTML.

[![npm](https://img.shields.io/npm/v/@comfit/css.svg)](https://www.npmjs.com/package/@comfit/css)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [npm](https://www.npmjs.com/package/@comfit/css)

## Installation

```sh
npm install @comfit/css
```

## Usage

With an initialized `@comfit/wasm` highlighter that has Rust loaded:

```ts
import "@comfit/css/catppuccin-mocha.css";

const html = highlighter.renderHtml("fn main() {}", "rust", {
  preClass: "comfit-catppuccin-mocha",
});
```

See [HTML and CSS](https://comfit.pinanek.me/docs/guides/html) and [Themes](https://comfit.pinanek.me/docs/guides/themes) for stylesheets, class prefixes, bracket palettes, and line emphasis.

## License

Licensed under the [MIT License](LICENSE).
