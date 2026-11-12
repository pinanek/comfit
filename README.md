<div align="center">
  <a href="https://comfit.pinanek.me">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/pinanek/comfit/refs/heads/main/logo/comfit-logo-dark.svg">
      <img src="https://raw.githubusercontent.com/pinanek/comfit/refs/heads/main/logo/comfit-logo-light.svg" alt="Comfit" width="420">
    </picture>
  </a>

  <p><strong>Tree-sitter syntax highlighting for Rust, JavaScript, and Python.</strong></p>

  <p>
    <a href="https://github.com/pinanek/comfit/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/pinanek/comfit/ci.yml?branch=main&amp;label=CI&amp;logo=github" alt="CI status"></a>
    <a href="https://github.com/pinanek/comfit/actions/workflows/security.yml"><img src="https://img.shields.io/github/actions/workflow/status/pinanek/comfit/security.yml?branch=main&amp;label=security&amp;logo=github" alt="Security audit status"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-16618b" alt="MIT license"></a>
  </p>

  <p>
    <a href="https://comfit.pinanek.me"><strong>Documentation</strong></a> ·
    <a href="https://comfit.pinanek.me/playground">Playground</a> ·
    <a href="#packages">Packages</a>
  </p>
</div>

## Packages

| Package                                              | Version                                                                                                               | Description                                                            |
| ---------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| [comfit](crates/comfit/README.md)                    | [![crates.io](https://img.shields.io/crates/v/comfit.svg)](https://crates.io/crates/comfit)                           | Comfit syntax highlighting library for Rust                            |
| [comfit-cli](crates/comfit_cli/README.md)            | [![crates.io](https://img.shields.io/crates/v/comfit-cli.svg)](https://crates.io/crates/comfit-cli)                   | Comfit command-line tool for highlighting and VS Code theme conversion |
| [comfit-core](crates/comfit_core/README.md)          | [![crates.io](https://img.shields.io/crates/v/comfit-core.svg)](https://crates.io/crates/comfit-core)                 | Comfit highlighting engine, theme model, and renderers                 |
| [comfit-grammars](crates/comfit_grammars/README.md)  | [![crates.io](https://img.shields.io/crates/v/comfit-grammars.svg)](https://crates.io/crates/comfit-grammars)         | Comfit parsers and queries for Rust                                    |
| [comfit-themes](crates/comfit_themes/README.md)      | [![crates.io](https://img.shields.io/crates/v/comfit-themes.svg)](https://crates.io/crates/comfit-themes)             | Comfit themes and VS Code theme conversion                             |
| [@comfit/node](npm/node/README.md)                   | [![npm](https://img.shields.io/npm/v/@comfit/node.svg)](https://www.npmjs.com/package/@comfit/node)                   | Comfit bindings for Node.js                                            |
| [@comfit/markdown-it](npm/markdown-it/README.md)     | [![npm](https://img.shields.io/npm/v/@comfit/markdown-it.svg)](https://www.npmjs.com/package/@comfit/markdown-it)     | Comfit plugin for highlighting fenced code blocks with markdown-it     |
| [@comfit/rehype](npm/rehype/README.md)               | [![npm](https://img.shields.io/npm/v/@comfit/rehype.svg)](https://www.npmjs.com/package/@comfit/rehype)               | Comfit plugin for highlighting code blocks with rehype                 |
| [@comfit/wasm](npm/wasm/README.md)                   | [![npm](https://img.shields.io/npm/v/@comfit/wasm.svg)](https://www.npmjs.com/package/@comfit/wasm)                   | Comfit WebAssembly bindings for browsers and Node.js                   |
| [@comfit/wasm-grammars](npm/wasm-grammars/README.md) | [![npm](https://img.shields.io/npm/v/@comfit/wasm-grammars.svg)](https://www.npmjs.com/package/@comfit/wasm-grammars) | Comfit grammar assets and queries for WebAssembly                      |
| [@comfit/css](npm/css/README.md)                     | [![npm](https://img.shields.io/npm/v/@comfit/css.svg)](https://www.npmjs.com/package/@comfit/css)                     | Comfit theme stylesheets for class-based HTML                          |
| [comfit-hl](python/comfit/README.md)                 | [![PyPI](https://img.shields.io/pypi/v/comfit-hl.svg)](https://pypi.org/project/comfit-hl/)                           | Comfit bindings for Python                                             |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for local setup, checks, and documentation workflows. `just init` installs local Git hooks.

To build and install packages, see [Build from source](https://comfit.pinanek.me/docs/development/build-from-source).

## License

Licensed under the [MIT License](LICENSE).

Third-party parsers, queries, themes, and runtimes retain their original licenses.
