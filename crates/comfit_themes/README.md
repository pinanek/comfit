# comfit-themes

Comfit themes and optional VS Code theme conversion.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [crates.io](https://crates.io/crates/comfit-themes)

## Installation

```sh
cargo add comfit-themes --features catppuccin-mocha
```

## Usage

```rust
fn main() {
    let theme = comfit_themes::catppuccin_mocha::theme();
    let style = theme.style("function.call");
    println!("{:?}", style.fg);
}
```

See the [theme guide](https://comfit.pinanek.me/docs/guides/themes) and [API reference](https://docs.rs/comfit-themes/latest/comfit_themes/) for theme selection and conversion.

## License

Licensed under the [MIT License](LICENSE).
