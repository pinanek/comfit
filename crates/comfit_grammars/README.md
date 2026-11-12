# comfit-grammars

Comfit parsers and queries for Rust, selected through Cargo features.

[![crates.io](https://img.shields.io/crates/v/comfit-grammars.svg)](https://crates.io/crates/comfit-grammars)
[![docs](https://img.shields.io/docsrs/comfit-grammars?label=docs)](https://docs.rs/comfit-grammars)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [crates.io](https://crates.io/crates/comfit-grammars)

## Installation

```sh
cargo add comfit-grammars --features rust
```

Also add a compatible Tree-sitter runtime to your dependencies:

```toml
tree-sitter = "0.27.2"
```

## Usage

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let language = comfit_grammars::rust::LANGUAGE.into();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language)?;
    let tree = parser.parse("fn main() {}", None).expect("parse failed");
    println!("{}", tree.root_node().to_sexp());
    Ok(())
}
```

See the [API reference](https://docs.rs/comfit-grammars/latest/comfit_grammars/) for the catalog and queries, or [Build from source](https://comfit.pinanek.me/docs/development/build-from-source) for grammar preparation.

## License

Licensed under the [MIT License](LICENSE).

Bundled parser licenses and query attribution are included in
[THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt).
