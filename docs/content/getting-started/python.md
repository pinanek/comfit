---
title: "Python"
description: "Install Comfit for Python to highlight source code, inspect tokens, and render HTML or ANSI output."
---

# Python

Install `comfit-hl` to highlight source with bundled languages and themes. The Python module is named `comfit`. Python 3.11 or later is required.

## Install

<CodeTabs label="Package manager">
<CodeTab label="uv">

```sh
# For a new project, run uv init first.
uv add comfit-hl
```

</CodeTab>
<CodeTab label="pip">

```sh
python -m pip install comfit-hl
```

</CodeTab>
</CodeTabs>

The package includes a native extension. To compile it from a checkout, follow [Build from source](../development/build-from-source.md#python).

## Render HTML

```python title="highlight.py"
import comfit

config = comfit.RenderHtmlConfig(style=comfit.RenderHtmlStyle.Inline)
html = comfit.render_html(
    'print("Hello, Comfit!")',
    "python",
    "catppuccin-mocha",
    config,
)
print(html)
```

Save the example as `highlight.py`. Run `uv run python highlight.py` in a uv project, or `python highlight.py` in the environment where you installed the package with pip.

The result is an escaped `<pre><code>` fragment with inline colors. Your application's CSS controls fonts and code block layout.

The default HTML style is classes. Select `Inline` as above for embedded colors, or add a [theme stylesheet](../guides/html.md#classes) for class output.

## Find languages and themes

```python
import comfit

print(comfit.languages())
print(comfit.themes())
```

Language aliases such as `py` and `rs` work wherever a language ID is accepted. Unknown languages and themes raise `ValueError`.

## Work with tokens or terminal text

```python
import comfit

lines = comfit.highlight('print("Hello")', "python")
for line in lines:
    for token in line.tokens:
        print(token.text, token.highlight)

ansi = comfit.render_ansi('print("Hello")', "python", "catppuccin-mocha")
print(ansi)
```

[Lines and tokens](../guides/tokens.md) describes the result fields. For source that changes repeatedly, use a [document session](../guides/documents.md).

Parsing and rendering release the GIL, but each function is synchronous: the calling Python thread waits for its result. Each calling thread reuses its own highlighter.

See the [Python API reference](../api-references/python/index.md) for configuration and error behavior.
