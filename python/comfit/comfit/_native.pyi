"""Types for syntax highlighting, rendering, and bundled languages and themes."""

from typing import ClassVar

class RenderHtmlStyle:
    """How token styles are rendered. Rendering requires a theme ID."""

    #: Embed literal theme styles in token elements.
    Inline: ClassVar[RenderHtmlStyle]
    #: Emit capture classes for a caller-provided stylesheet.
    Class: ClassVar[RenderHtmlStyle]
    #: Reference caller-provided CSS variables instead of theme values.
    Variables: ClassVar[RenderHtmlStyle]

class HighlightConfig:
    """Options for line emphasis and bracket-depth collection."""

    #: One-based source lines to emphasize. Defaults to an empty list.
    highlighted_lines: list[int]
    #: Collect bracket depths. Defaults to True.
    rainbow: bool
    def __init__(
        self, *, highlighted_lines: list[int] | None = None, rainbow: bool = True
    ) -> None:
        """Create highlighting options. Bracket-depth collection is enabled by default."""
        ...

class RenderHtmlConfig:
    """Highlighting and HTML options, defaulting to capture classes and rainbow depths."""

    #: One-based source lines to emphasize. Defaults to an empty list.
    highlighted_lines: list[int]
    #: Collect bracket depths. Defaults to True.
    rainbow: bool
    #: Outer tag name. Defaults to "pre".
    pre_tag: str
    #: Line tag name. Defaults to "span".
    line_tag: str
    #: Optional class on the outer element, escaped as attribute text.
    pre_class: str | None
    #: Prefix for token, rainbow, and line classes. Defaults to an empty string.
    class_prefix: str
    #: Number of bracket colors to cycle through. Zero preserves depth without a theme palette.
    rainbow_colors: int
    #: How token styles are rendered. Defaults to RenderHtmlStyle.Class.
    style: RenderHtmlStyle
    def __init__(
        self,
        *,
        highlighted_lines: list[int] | None = None,
        rainbow: bool = True,
        pre_class: str | None = None,
        class_prefix: str = "",
        rainbow_colors: int = 0,
        style: RenderHtmlStyle = RenderHtmlStyle.Class,
        pre_tag: str = "pre",
        line_tag: str = "span",
    ) -> None:
        """Create HTML options. Capture classes and bracket-depth collection are enabled by default."""
        ...

class Token:
    """Owned source text with an optional capture style and bracket depth."""

    @property
    def text(self) -> str:
        """Source text without newline separators."""
        ...
    @property
    def highlight(self) -> str | None:
        """Capture name, or None for unstyled text."""
        ...
    @property
    def rainbow(self) -> int | None:
        """Zero-based bracket depth, or None when no depth was collected."""
        ...

class Line:
    """A source line without its newline separator. Empty lines are included."""

    @property
    def number(self) -> int:
        """One-based source line number."""
        ...
    @property
    def highlighted(self) -> bool:
        """Whether this line is selected for emphasis."""
        ...
    @property
    def tokens(self) -> list[Token]:
        """Tokens in source order. Their boundaries may differ from syntax node boundaries."""
        ...

def highlight(
    source: str, language: str, config: HighlightConfig | None = None
) -> list[Line]:
    """Highlight source into owned lines and tokens, releasing the GIL during parsing.

    Results remain usable after later calls. Empty sources produce one line.
    A trailing newline produces a final empty line.

    Raises:
        ValueError: The language is unknown or a source range is invalid.
        RuntimeError: The backend fails or injection nesting exceeds 32 layers.
    """
    ...
def render_html(
    source: str, language: str, theme: str, config: RenderHtmlConfig | None = None
) -> str:
    """Render escaped HTML with a bundled theme.

    Class styles need a stylesheet. Variable styles need CSS variables.
    Parsing and rendering release the GIL.

    Raises:
        ValueError: A theme, language, or source range is invalid.
        RuntimeError: The backend fails or injection nesting exceeds 32 layers.
    """
    ...
def render_ansi(
    source: str, language: str, theme: str, config: HighlightConfig | None = None
) -> str:
    """Render ANSI-styled source with a bundled theme.

    Controls other than tabs and newlines are escaped. The core renderer does not
    add CLI gutters, wrapping, padding, or color detection. Parsing and rendering
    release the GIL.

    Raises:
        ValueError: A theme, language, or source range is invalid.
        RuntimeError: The backend fails or injection nesting exceeds 32 layers.
    """
    ...
def languages() -> list[str]:
    """Return bundled language IDs. Rendering also accepts language aliases."""
    ...
def themes() -> list[str]:
    """Return bundled theme IDs without initializing their palettes."""
    ...

class Document:
    """Incremental source document sharing compiled grammars with the calling thread.

    Use create_document() to construct a session. A session snapshots the currently
    loaded languages. Dispose it explicitly or use it as a context manager.
    """

    @property
    def source(self) -> str: ...
    def update(self, source: str) -> None:
        """Incrementally parse changed source, releasing the GIL. If the source is unchanged, do nothing."""
        ...
    def highlight(self, config: HighlightConfig | None = None) -> HighlightResult:
        """Return an owned snapshot. Changes to source or rainbow options invalidate the cache."""
        ...
    def dispose(self) -> None:
        """Release retained trees. Existing snapshots remain valid. Repeated calls are harmless."""
        ...
    def __enter__(self) -> Document: ...
    def __exit__(self, exc_type: object, exc_value: object, traceback: object) -> None: ...

class HighlightResult:
    """Owned lines and tokens, independent of document updates or disposal."""

    @property
    def lines(self) -> list[Line]: ...
    def render_html(self, theme: str, config: RenderHtmlConfig | None = None) -> str:
        """Render without parsing. The config can change line emphasis, but not bracket depths."""
        ...
    def render_ansi(self, theme: str) -> str:
        """Render the snapshot without parsing."""
        ...

def create_document(*, language: str, source: str) -> Document:
    """Create an incremental document using a bundled language ID or alias."""
    ...
