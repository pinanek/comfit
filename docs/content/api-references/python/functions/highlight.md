<a id="comfit.highlight"></a>

## `comfit.highlight`

```python
highlight(source: str, language: str, config: HighlightConfig | None = None) -> list[Line]
```

Highlight source into owned lines and tokens, releasing the GIL during parsing.

Results remain usable after later calls. Empty sources produce one line.
A trailing newline produces a final empty line.

**Raises:**

- <code>ValueError</code> – The language is unknown or a source range is invalid.
- <code>RuntimeError</code> – The backend fails or injection nesting exceeds 32 layers.
