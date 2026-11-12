# comfit-core

Comfit highlighting engine, theme model, and renderers.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [crates.io](https://crates.io/crates/comfit-core)

## Installation

```sh
cargo add comfit-core --features render-html
```

## Usage

```rust
use comfit_core::{Highlighted, Line, RenderHtmlConfig, Token};

fn main() {
    let highlighted = Highlighted {
        lines: vec![Line {
            number: 1,
            highlighted: false,
            tokens: vec![Token {
                text: "fn",
                highlight: Some("keyword.function"),
                rainbow: None,
            }],
        }],
    };
    println!(
        "{}",
        highlighted.render_html_classes(&RenderHtmlConfig::default())
    );
}
```

See the [API reference](https://docs.rs/comfit-core/latest/comfit_core/) for backend integration and the [HTML guide](https://comfit.pinanek.me/docs/guides/html) for rendering options.

## License

Licensed under the [MIT License](LICENSE).
