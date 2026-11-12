<a id="comfit.Token"></a>

## `comfit.Token`

Owned source text with an optional capture style and bracket depth.

**Attributes:**

- [**highlight**](#comfit.Token.highlight) (<code>str | None</code>) – Capture name, or None for unstyled text.
- [**rainbow**](#comfit.Token.rainbow) (<code>int | None</code>) – Zero-based bracket depth, or None when no depth was collected.
- [**text**](#comfit.Token.text) (<code>str</code>) – Source text without newline separators.

<a id="comfit.Token.highlight"></a>

### `comfit.Token.highlight`

```python
highlight: str | None
```

Capture name, or None for unstyled text.

<a id="comfit.Token.rainbow"></a>

### `comfit.Token.rainbow`

```python
rainbow: int | None
```

Zero-based bracket depth, or None when no depth was collected.

<a id="comfit.Token.text"></a>

### `comfit.Token.text`

```python
text: str
```

Source text without newline separators.
