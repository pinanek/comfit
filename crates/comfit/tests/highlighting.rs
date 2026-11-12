//! Exercise native highlighting, language registration, and injection ranges.

#![cfg(feature = "rust")]

use comfit::{Error, HighlightConfig, Highlighter, Queries};

#[test]
fn reused_highlighter_preserves_unicode_and_empty_lines() -> Result<(), Error> {
    let mut highlighter = Highlighter::new()?;

    for source in [
        "fn café() { let value = [1, (2)]; }\n\n",
        "",
        "\n",
        "fn main() { println!(\"😀\"); }",
    ] {
        let highlighted = highlighter.highlight(
            source,
            "rs",
            &HighlightConfig {
                highlighted_lines: vec![2, 1, 2, 0],
                rainbow: true,
            },
        )?;
        let restored = highlighted
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

        assert_eq!(restored, source);
        assert_eq!(highlighted.lines.len(), source.split('\n').count());
        assert!(highlighted.lines[0].highlighted);

        if source.contains('[') {
            assert!(
                highlighted.lines[0]
                    .tokens
                    .iter()
                    .any(|token| token.rainbow.is_some())
            );
        }
    }

    Ok(())
}

#[test]
fn document_updates_preserve_source_and_snapshots() -> Result<(), Error> {
    let mut highlighter = Highlighter::new()?;
    let original = "fn café() { println!(\"😀\"); }\n";
    let config = HighlightConfig::default();
    let mut document = highlighter.create_document("rs", original)?;
    let snapshot = document.highlight(&config)?;

    for source in [
        "fn café() { println!(\"😁\"); }\n",
        "",
        "\n",
        "fn longer_function() { let values = [1, 2, 3]; }\n\n",
        original,
        original,
    ] {
        document.update(source)?;
        assert_eq!(document.source(), source);
        assert_eq!(
            document.highlight(&config)?.as_highlighted(),
            highlighter.highlight(source, "rs", &config)?,
        );
    }

    drop(document);
    assert_eq!(
        snapshot.as_highlighted(),
        highlighter.highlight(original, "rs", &config)?,
    );

    Ok(())
}

#[test]
fn failed_registration_does_not_reserve_names() -> Result<(), Error> {
    let mut highlighter = Highlighter::new()?;
    let language = comfit::grammars::rust::LANGUAGE.into();

    assert!(matches!(
        highlighter.load(
            "custom",
            &["custom-alias"],
            language,
            Queries {
                highlights: "(not_a_rust_node) @variable",
                ..Default::default()
            }
        ),
        Err(Error::Query(_)),
    ));
    assert!(!highlighter.contains("custom"));
    assert!(!highlighter.contains("custom-alias"));

    highlighter.load(
        "custom",
        &["custom-alias"],
        comfit::grammars::rust::LANGUAGE.into(),
        Queries::default(),
    )?;
    assert!(highlighter.contains("custom-alias"));

    Ok(())
}

#[cfg(feature = "markdown")]
#[test]
fn injections_preserve_ranges_and_skip_unloaded_languages() -> Result<(), Error> {
    let mut highlighter = Highlighter::new()?;
    let source =
        "😀\n```rust\nfn café() { let x = [1]; }\n```\n```not-loaded\nfn unknown() {}\n```\n";
    assert!(!highlighter.contains("not-loaded"));
    let highlighted = highlighter.highlight(
        source,
        "markdown",
        &HighlightConfig {
            rainbow: true,
            ..Default::default()
        },
    )?;
    let rust = &highlighted.lines[2];

    assert!(
        rust.tokens
            .iter()
            .any(|token| token.text == "fn" && token.highlight == Some("keyword.function"))
    );
    assert!(
        !highlighted.lines[5]
            .tokens
            .iter()
            .any(|token| token.highlight == Some("keyword.function"))
    );

    Ok(())
}
