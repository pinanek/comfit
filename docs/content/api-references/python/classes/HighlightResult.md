<a id="comfit.HighlightResult"></a>

## `comfit.HighlightResult`

Owned lines and tokens, independent of document updates or disposal.

**Functions:**

- [**render_ansi**](#comfit.HighlightResult.render_ansi) – Render the snapshot without parsing.
- [**render_html**](#comfit.HighlightResult.render_html) – Render without parsing. The config can change line emphasis, but not bracket depths.

**Attributes:**

- [**lines**](#comfit.HighlightResult.lines) (<code>list[[Line](Line.md#comfit.Line)]</code>) –

<a id="comfit.HighlightResult.lines"></a>

### `comfit.HighlightResult.lines`

```python
lines: list[Line]
```

<a id="comfit.HighlightResult.render_ansi"></a>

### `comfit.HighlightResult.render_ansi`

```python
render_ansi(theme: str) -> str
```

Render the snapshot without parsing.

<a id="comfit.HighlightResult.render_html"></a>

### `comfit.HighlightResult.render_html`

```python
render_html(theme: str, config: RenderHtmlConfig | None = None) -> str
```

Render without parsing. The config can change line emphasis, but not bracket depths.
