# Comfit

Tree-sitter syntax highlighting for Rust, JavaScript, and Python.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [Packages](#packages) · [Getting started](#getting-started)

## Getting started

```sh
cargo install comfit-cli
comfit ansi main.rs --theme catppuccin-mocha --line-numbers
comfit html main.rs --theme catppuccin-mocha -o code.html
```

Use `--language` when reading stdin. For application code, start with the [Rust](https://comfit.pinanek.me/docs/getting-started/rust), [Node.js](https://comfit.pinanek.me/docs/getting-started/node), [browser](https://comfit.pinanek.me/docs/getting-started/browser), or [Python](https://comfit.pinanek.me/docs/getting-started/python) guide.

## Packages

| Package                                              | Description                                                            |
| ---------------------------------------------------- | ---------------------------------------------------------------------- |
| [comfit](crates/comfit/README.md)                    | Comfit syntax highlighting library for Rust                            |
| [comfit-cli](crates/comfit_cli/README.md)            | Comfit command-line tool for highlighting and VS Code theme conversion |
| [comfit-core](crates/comfit_core/README.md)          | Comfit highlighting engine, theme model, and renderers                 |
| [comfit-grammars](crates/comfit_grammars/README.md)  | Comfit parsers and queries for Rust                                    |
| [comfit-themes](crates/comfit_themes/README.md)      | Comfit themes and VS Code theme conversion                             |
| [@comfit/node](npm/node/README.md)                   | Comfit bindings for Node.js                                            |
| [@comfit/markdown-it](npm/markdown-it/README.md)     | Comfit plugin for highlighting fenced code blocks with markdown-it     |
| [@comfit/rehype](npm/rehype/README.md)               | Comfit plugin for highlighting code blocks with rehype                 |
| [@comfit/wasm](npm/wasm/README.md)                   | Comfit WebAssembly bindings for browsers and Node.js                   |
| [@comfit/wasm-grammars](npm/wasm-grammars/README.md) | Comfit grammar assets and queries for WebAssembly                      |
| [@comfit/css](npm/css/README.md)                     | Comfit theme stylesheets for class-based HTML                          |
| [comfit for Python](python/comfit/README.md)         | Comfit bindings for Python                                             |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for local setup, checks, and documentation workflows. `just init` installs local Git hooks.

To build and install packages, see [Build from source](https://comfit.pinanek.me/docs/development/build-from-source).

## License

Licensed under the [MIT License](LICENSE).

Bundled components retain their own licenses. See the [grammar and query notices](npm/node/THIRD_PARTY_NOTICES.txt) and the [WASM license notes](npm/wasm/README.md#license).
