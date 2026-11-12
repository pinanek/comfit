<a id="comfit.render_html"></a>

## `comfit.render_html`

```python
render_html(source: str, language: str, theme: str, config: RenderHtmlConfig | None = None) -> str
```

Render escaped HTML with a bundled theme.

Class styles need a stylesheet. Variable styles need CSS variables.
Parsing and rendering release the GIL.

**Raises:**

- <code>ValueError</code> – A theme, language, or source range is invalid.
- <code>RuntimeError</code> – The backend fails or injection nesting exceeds 32 layers.
