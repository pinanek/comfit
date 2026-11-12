# comfit

Comfit syntax highlighting library for Rust.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [crates.io](https://crates.io/crates/comfit)

## Installation

```sh
cargo add comfit --features rust,render-html
```

## Usage

```rust
fn main() -> Result<(), comfit::Error> {
    let mut highlighter = comfit::Highlighter::new()?;
    let highlighted =
        highlighter.highlight("fn main() {}", "rust", &comfit::HighlightConfig::default())?;
    let html = highlighted.render_html_classes(&comfit::RenderHtmlConfig::default());
    println!("{html}");
    Ok(())
}
```

See the [Rust guide](https://comfit.pinanek.me/docs/getting-started/rust) for themes, tokens, and document sessions, or browse the [API reference](https://docs.rs/comfit/latest/comfit/).

## License

Licensed under the [MIT License](LICENSE).
