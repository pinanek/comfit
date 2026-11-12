//! Check HTML escaping, theme precedence, and CSS color rendering.

#![cfg(feature = "render-html")]

use std::collections::BTreeMap;

use comfit_core::{
    Color, Highlighted, Line, Modifier, RenderHtmlConfig, RenderHtmlStyle, Style, Theme, Token,
    Underline, UnderlineStyle,
};

#[test]
fn class_html_preserves_escaping_empty_captures_and_rainbow_wrapping() {
    let highlighted = Highlighted {
        lines: vec![Line {
            number: 1,
            highlighted: true,
            tokens: vec![
                Token {
                    text: "<&\"'",
                    highlight: Some(".a..b"),
                    rainbow: Some(8),
                },
                Token {
                    text: "plain",
                    highlight: None,
                    rainbow: None,
                },
                Token {
                    text: "empty",
                    highlight: Some(""),
                    rainbow: None,
                },
            ],
        }],
    };
    let config = RenderHtmlConfig {
        class_prefix: "<&".into(),
        pre_class: Some("\"'".into()),
        rainbow_colors: 7,
        ..Default::default()
    };

    assert_eq!(
        highlighted.render_html_classes(&config),
        concat!(
            "<pre class=\"&quot;&#39;\"><code>",
            "<span class=\"&lt;&amp;line &lt;&amp;line-highlight\">",
            "<span class=\"&lt;&amp; &lt;&amp;a &lt;&amp;a- &lt;&amp;a--b ",
            "&lt;&amp;rainbow &lt;&amp;rainbow-1\">&lt;&amp;&quot;&#39;</span>",
            "<span>plain</span><span class=\"&lt;&amp;\">empty</span>",
            "</span></code></pre>"
        ),
    );

    let inline_without_theme = RenderHtmlConfig {
        style: RenderHtmlStyle::Inline,
        ..config.clone()
    };
    assert_eq!(
        highlighted.render_html_classes(&inline_without_theme),
        highlighted.render_html_classes(&config),
    );
}

#[test]
fn inline_html_keeps_theme_and_line_styles_separate() {
    let theme = Theme {
        default: Style {
            fg: Some(Color::Indexed(1)),
            ..Default::default()
        },
        styles: BTreeMap::from([(
            "keyword".into(),
            Style {
                modifiers: vec![Modifier::Bold],
                ..Default::default()
            },
        )]),
        line_highlight: Style {
            bg: Some(Color::Indexed(2)),
            ..Default::default()
        },
        rainbow: vec![Color::Indexed(4)],
        ..Default::default()
    };
    let highlighted = Highlighted {
        lines: vec![
            Line {
                number: 1,
                highlighted: true,
                tokens: vec![Token {
                    text: "<&",
                    highlight: Some("keyword.function"),
                    rainbow: Some(100),
                }],
            },
            Line {
                number: 2,
                highlighted: false,
                tokens: vec![],
            },
        ],
    };

    assert_eq!(
        highlighted.render_html(
            &theme,
            &RenderHtmlConfig {
                style: RenderHtmlStyle::Inline,
                ..Default::default()
            },
        ),
        concat!(
            "<pre style=\"color:#800000;\"><code>",
            "<span class=\"line line-highlight\" style=\"background-color:#008000;\">",
            "<span style=\"color:#000080;font-weight:bold;\">&lt;&amp;</span>",
            "</span>\n<span class=\"line\"></span></code></pre>"
        ),
    );
}

#[test]
fn html_preserves_carriage_returns_across_token_and_line_boundaries() {
    let highlighted = Highlighted {
        lines: vec![
            Line {
                number: 1,
                highlighted: false,
                tokens: vec![Token {
                    text: "text\r",
                    highlight: None,
                    rainbow: None,
                }],
            },
            Line {
                number: 2,
                highlighted: false,
                tokens: vec![],
            },
        ],
    };
    for style in [
        RenderHtmlStyle::Class,
        RenderHtmlStyle::Inline,
        RenderHtmlStyle::Variables,
    ] {
        let html = highlighted.render_html(
            &Theme::default(),
            &RenderHtmlConfig {
                style,
                ..Default::default()
            },
        );
        assert!(html.contains("text&#13;</span></span>\n"));
        assert!(!html.contains('\r'));
    }
}

#[test]
fn css_preserves_declaration_order_and_terminal_colors() {
    let style = Style {
        fg: Some(Color::Indexed(232)),
        bg: Some(Color::Default),
        modifiers: vec![Modifier::Reversed, Modifier::Bold, Modifier::CrossedOut],
        underline: Some(Underline {
            color: Some(Color::Indexed(255)),
            style: Some(UnderlineStyle::Curl),
        }),
    };

    assert_eq!(
        style.to_css(),
        concat!(
            "color:inherit;background-color:#080808;font-weight:bold;",
            "text-decoration-line:line-through;",
            "text-decoration-line:underline line-through;",
            "text-decoration-style:wavy;text-decoration-color:#eeeeee;"
        ),
    );
    assert_eq!(Color::Indexed(16).to_css(), "#000000");
    assert_eq!(Color::Indexed(231).to_css(), "#ffffff");
}
