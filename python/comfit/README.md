# Comfit for Python

Comfit bindings for Python.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Documentation](https://comfit.pinanek.me) · [PyPI](https://pypi.org/project/comfit/)

## Installation

```sh
python -m pip install comfit
```

## Usage

```python
import comfit

config = comfit.RenderHtmlConfig(style=comfit.RenderHtmlStyle.Inline)
html = comfit.render_html(
    'fn main() {}',
    "rust",
    "catppuccin-mocha",
    config,
)
```

See the [Python guide](https://comfit.pinanek.me/docs/getting-started/python) for tokens, ANSI output, and document sessions.

## License

Licensed under the [MIT License](LICENSE).

Bundled parser licenses and query attribution are included in [THIRD_PARTY_NOTICES.txt](comfit/THIRD_PARTY_NOTICES.txt).
