<a id="comfit.Document"></a>

## `comfit.Document`

Incremental source document sharing compiled grammars with the calling thread.

Use create_document() to construct a session. A session snapshots the currently
loaded languages. Dispose it explicitly or use it as a context manager.

**Functions:**

- [**dispose**](#comfit.Document.dispose) – Release retained trees. Existing snapshots remain valid. Repeated calls are harmless.
- [**highlight**](#comfit.Document.highlight) – Return an owned snapshot. Changes to source or rainbow options invalidate the cache.
- [**update**](#comfit.Document.update) – Incrementally parse changed source, releasing the GIL. If the source is unchanged, do nothing.

**Attributes:**

- [**source**](#comfit.Document.source) (<code>str</code>) –

<a id="comfit.Document.dispose"></a>

### `comfit.Document.dispose`

```python
dispose() -> None
```

Release retained trees. Existing snapshots remain valid. Repeated calls are harmless.

<a id="comfit.Document.highlight"></a>

### `comfit.Document.highlight`

```python
highlight(config: HighlightConfig | None = None) -> HighlightResult
```

Return an owned snapshot. Changes to source or rainbow options invalidate the cache.

<a id="comfit.Document.source"></a>

### `comfit.Document.source`

```python
source: str
```

<a id="comfit.Document.update"></a>

### `comfit.Document.update`

```python
update(source: str) -> None
```

Incrementally parse changed source, releasing the GIL. If the source is unchanged, do nothing.
