//! Shared fixtures exercise the same public contract in Rust, Node, WASM, and Python.

#![cfg(all(
    feature = "rust",
    feature = "javascript",
    feature = "markdown",
    feature = "markdown_inline"
))]

use comfit::{HighlightConfig, Highlighter};
use serde::Deserialize;

type ExpectedToken = (String, Option<String>, Option<usize>);

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    name: String,
    source: String,
    language: String,
    highlighted_lines: Vec<usize>,
    rainbow: bool,
    tokens: Vec<Vec<ExpectedToken>>,
}

#[test]
fn shared_highlighting_contract() -> Result<(), Box<dyn std::error::Error>> {
    let fixtures: Vec<Fixture> = serde_json::from_str(include_str!("fixtures/highlighting.json"))?;
    let mut highlighter = Highlighter::new()?;

    for fixture in fixtures {
        let config = HighlightConfig {
            highlighted_lines: fixture.highlighted_lines,
            rainbow: fixture.rainbow,
        };
        let mut document = highlighter.create_document(&fixture.language, "")?;
        document.update(&fixture.source)?;
        let result = highlighter.highlight(&fixture.source, &fixture.language, &config)?;
        let tokens: Vec<Vec<_>> = result
            .lines
            .iter()
            .map(|line| {
                line.tokens
                    .iter()
                    .map(|token| {
                        (
                            token.text.to_owned(),
                            token.highlight.map(str::to_owned),
                            token.rainbow,
                        )
                    })
                    .collect()
            })
            .collect();
        assert_eq!(tokens, fixture.tokens, "{}", fixture.name);
        assert_eq!(
            result.lines.len(),
            fixture.source.split('\n').count(),
            "{}",
            fixture.name
        );
        for (index, line) in result.lines.iter().enumerate() {
            assert_eq!(line.number, index + 1, "{}", fixture.name);
            assert_eq!(
                line.highlighted,
                config.highlighted_lines.contains(&(index + 1)),
                "{}",
                fixture.name
            );
        }
        let restored = result
            .lines
            .iter()
            .map(|line| {
                line.tokens
                    .iter()
                    .map(|token| token.text)
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(restored, fixture.source, "{}", fixture.name);

        assert_eq!(
            document.highlight(&config)?.as_highlighted(),
            result,
            "{}",
            fixture.name
        );
    }

    Ok(())
}
