"""Syntax highlighting with bundled Tree-sitter grammars and themes."""

from ._native import (
    Document,
    HighlightConfig,
    HighlightResult,
    create_document,
    Line,
    RenderHtmlConfig,
    RenderHtmlStyle,
    Token,
    highlight,
    languages,
    render_ansi,
    render_html,
    themes,
)

__all__ = [
    "Document",
    "HighlightConfig",
    "HighlightResult",
    "create_document",
    "Line",
    "RenderHtmlConfig",
    "RenderHtmlStyle",
    "Token",
    "highlight",
    "languages",
    "render_ansi",
    "render_html",
    "themes",
]
