---
title: "Rust"
description: "Add Comfit to a Rust application, select language features, and highlight source code."
---

# Rust

Use `comfit` to highlight source in a Rust application. Cargo features select the languages and renderers included in your build. Building grammars requires Rust with Cargo and C/C++ build tools.

## Add the dependencies

```sh
cargo add comfit --features rust,render-html
cargo add comfit-themes --features catppuccin-mocha
```

If you use a source checkout, [prepare its grammar data](../development/build-from-source.md#prepare-the-grammars) before building.

No languages are enabled by default. This example selects Rust.

Add `python`, `javascript`, or other language features as needed. `all-languages` enables every bundled language. See [Languages](../guides/languages.md) for supported languages, embedded-language dependencies, and direct access to parsers and queries.

## Render HTML

```rust title="src/main.rs"
fn main() -> Result<(), comfit::Error> {
    let mut highlighter = comfit::Highlighter::new()?;
    let highlighted = highlighter.highlight(
        "fn main() { let answer = 42; }",
        "rust",
        &comfit::HighlightConfig::default(),
    )?;
    let theme = comfit_themes::catppuccin_mocha::theme();
    let html = highlighted.render_html(
        theme,
        &comfit::RenderHtmlConfig {
            style: comfit::RenderHtmlStyle::Inline,
            ..Default::default()
        },
    );

    println!("{html}");
    Ok(())
}
```

Run `cargo run` to print an escaped HTML fragment with inline colors. Use `render_html_classes` instead if you want to [supply a stylesheet](../guides/html.md#classes).

## Reuse the highlighter

`Highlighter::new` registers the enabled languages. Queries compile on first use and are reused by later calls. Keep one highlighter rather than creating it for every snippet.

The result of `highlight` borrows both the source and the highlighter. Finish using it before calling `highlight` again. For owned snapshots and source edits, create a [document session](../guides/documents.md).

## Render terminal text

Enable `render-ansi` alongside your language features. With a highlighted result and a theme, call `highlighted.render_ansi(theme)`.

The library renderer emits ANSI styles but does not choose a terminal width or add line numbers. Use the [CLI](cli.md) when you need that layout behavior.

See the [Rust API reference](../api-references/rust.md) for public types and the [theme guide](../guides/themes.md) for custom palettes.
