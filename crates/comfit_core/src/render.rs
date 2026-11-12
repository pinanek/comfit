//! Optional output renderers.

#[cfg(feature = "render-ansi")]
pub(crate) mod ansi;

#[cfg(feature = "render-html")]
pub mod html;

pub(crate) trait RenderLine {
    fn highlighted(&self) -> bool;
    fn tokens(&self) -> impl Iterator<Item = crate::Token<'_>>;
}

impl RenderLine for crate::Line<'_> {
    fn highlighted(&self) -> bool {
        self.highlighted
    }

    fn tokens(&self) -> impl Iterator<Item = crate::Token<'_>> {
        self.tokens.iter().cloned()
    }
}

impl RenderLine for crate::OwnedLine {
    fn highlighted(&self) -> bool {
        self.highlighted
    }

    fn tokens(&self) -> impl Iterator<Item = crate::Token<'_>> {
        self.tokens.iter().map(|token| crate::Token {
            text: &token.text,
            highlight: token.highlight.as_deref(),
            rainbow: token.rainbow,
        })
    }
}
