# comfit-cli

Comfit command-line tool for highlighting and VS Code theme conversion.

[![crates.io](https://img.shields.io/crates/v/comfit-cli.svg)](https://crates.io/crates/comfit-cli)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [crates.io](https://crates.io/crates/comfit-cli)

## Installation

Download a prebuilt binary from [GitHub Releases](https://github.com/pinanek/comfit/releases), or install with Cargo:

```sh
cargo install comfit-cli
```

## Usage

```sh
comfit ansi main.rs --theme catppuccin-mocha --line-numbers
comfit html main.rs --theme catppuccin-mocha -o code.html
comfit ansi --list-languages
comfit ansi --list-themes
```

See the [CLI guide](https://comfit.pinanek.me/docs/getting-started/cli) for wrapping, line selection, HTML styles, and theme conversion.

## License

Licensed under the [MIT License](LICENSE).

Bundled parser licenses, runtime license, and query attribution are included in
[THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt).
