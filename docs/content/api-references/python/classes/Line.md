<a id="comfit.Line"></a>

## `comfit.Line`

A source line without its newline separator. Empty lines are included.

**Attributes:**

- [**highlighted**](#comfit.Line.highlighted) (<code>bool</code>) – Whether this line is selected for emphasis.
- [**number**](#comfit.Line.number) (<code>int</code>) – One-based source line number.
- [**tokens**](#comfit.Line.tokens) (<code>list[[Token](Token.md#comfit.Token)]</code>) – Tokens in source order. Their boundaries may differ from syntax node boundaries.

<a id="comfit.Line.highlighted"></a>

### `comfit.Line.highlighted`

```python
highlighted: bool
```

Whether this line is selected for emphasis.

<a id="comfit.Line.number"></a>

### `comfit.Line.number`

```python
number: int
```

One-based source line number.

<a id="comfit.Line.tokens"></a>

### `comfit.Line.tokens`

```python
tokens: list[Token]
```

Tokens in source order. Their boundaries may differ from syntax node boundaries.
