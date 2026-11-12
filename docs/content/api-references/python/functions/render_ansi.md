<a id="comfit.render_ansi"></a>

## `comfit.render_ansi`

```python
render_ansi(source: str, language: str, theme: str, config: HighlightConfig | None = None) -> str
```

Render ANSI-styled source with a bundled theme.

Controls other than tabs and newlines are escaped. The core renderer does not
add CLI gutters, wrapping, padding, or color detection. Parsing and rendering
release the GIL.

**Raises:**

- <code>ValueError</code> – A theme, language, or source range is invalid.
- <code>RuntimeError</code> – The backend fails or injection nesting exceeds 32 layers.
