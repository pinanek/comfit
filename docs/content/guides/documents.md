---
title: "Updating source"
description: "Reuse Comfit documents to update source incrementally and keep highlighting in sync with edits."
---

# Updating source

Use a document session when highlighting the same source after edits. Unlike a one-shot call, a document retains its source and syntax trees so later edits can be parsed incrementally. It shares compiled queries with the highlighter.

The usual sequence is to create a document, update its source, highlight it, and render the result. Keep the document across edits. Each result is a snapshot that can be rendered independently.

For unrelated snippets, the one-shot functions in the quickstarts are usually simpler.

## Create and update a document

<CodeTabs label="Document session API">
<CodeTab label="Node.js">

Dispose the document when finished. Result objects do not need disposal.

```javascript
import {
  createDocument,
  RenderHtmlConfig,
  RenderHtmlStyle,
} from "@comfit/node";

const document = createDocument({ language: "rust", source: "fn main() {}" });
try {
  document.update("fn main() { let answer = 42; }");
  const result = document.highlight();

  const config = new RenderHtmlConfig();
  config.style = RenderHtmlStyle.Inline;
  const darkHtml = result.renderHtml("catppina-dark", config);
  const lightHtml = result.renderHtml("catppina-light", config);
} finally {
  document.dispose();
}
```

</CodeTab>
<CodeTab label="Browser">

Use an initialized highlighter with Rust loaded and a bundler that supports CSS imports. Dispose each result and the document when finished.

```typescript
import "@comfit/css/catppina-dark.css";
import "@comfit/css/catppina-light.css";

const document = highlighter.createDocument({
  language: "rust",
  source: "fn main() {}",
});
try {
  document.update("fn main() { let answer = 42; }");
  const result = document.highlight();
  try {
    const darkHtml = result.renderHtml({ preClass: "comfit-catppina-dark" });
    const lightHtml = result.renderHtml({ preClass: "comfit-catppina-light" });
  } finally {
    result.dispose();
  }
} finally {
  document.dispose();
}
```

</CodeTab>
<CodeTab label="Rust">

Enable `rust` and `render-html` on `comfit`, and `catppina-dark` and `catppina-light` on `comfit-themes`.

Documents and results are released when dropped.

```rust
let highlighter = comfit::Highlighter::new()?;
let mut document = highlighter.create_document("rust", "fn main() {}")?;
document.update("fn main() { let answer = 42; }")?;
let result = document.highlight(&comfit::HighlightConfig::default())?;
let config = comfit::RenderHtmlConfig {
    style: comfit::RenderHtmlStyle::Inline,
    ..Default::default()
};
let dark_html = result.render_html(comfit_themes::catppina_dark::theme(), &config);
let light_html = result.render_html(comfit_themes::catppina_light::theme(), &config);
```

</CodeTab>
<CodeTab label="Python">

A context manager disposes the document automatically. You can also call `dispose` explicitly. Result objects do not need disposal.

```python
import comfit

with comfit.create_document(language="rust", source="fn main() {}") as document:
    document.update("fn main() { let answer = 42; }")
    result = document.highlight()

    config = comfit.RenderHtmlConfig(style=comfit.RenderHtmlStyle.Inline)
    dark_html = result.render_html("catppina-dark", config)
    light_html = result.render_html("catppina-light", config)
```

</CodeTab>
</CodeTabs>

Each example updates the source once, then renders the same result with two themes. The second render does not parse or highlight the source again.

## Know what is reused

Parsing, highlighting, and rendering reuse different work:

| Operation                          | Work performed                                                                                                                   |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Update the source                  | Derive a UTF-8-safe edit and incrementally reparse the root and compatible embedded-language trees. Unchanged source is a no-op. |
| Highlight after an edit            | Run highlighting queries on the updated trees.                                                                                   |
| Change the rainbow option          | Invalidate cached highlighting and rerun queries on the next highlight.                                                          |
| Change only the emphasized lines   | Reuse cached highlighting.                                                                                                       |
| Render a result with another theme | Reuse the result without parsing or running queries.                                                                             |

Embedded-language layers update from parent to child. Compatible layers reuse their trees. New layers and edits at injection boundaries require fresh parsing.

Incremental parsing does not make query execution incremental.

## Keep snapshots and languages separate

Results own their source and captures, so later edits or document disposal do not invalidate a snapshot. Documents also remain usable after the original Rust or browser highlighter is released.

A document snapshots the registry's loaded languages at creation. Create another document to change the root language or use a different set of grammars.

Bracket depths are collected during highlighting. Rendering cannot add depths if collection was disabled.

## Release browser resources

Browser documents and results hold WASM resources. After creating a browser highlighter, use this cleanup pattern:

```typescript
const document = highlighter.createDocument({
  language: "rust",
  source: "fn main() {}",
});

try {
  const result = document.highlight();
  try {
    const html = result.renderHtml();
    const lines = result.lines;
  } finally {
    result.dispose();
  }
} finally {
  document.dispose();
}
```

Keep documents across edits, but dispose each result when you no longer need to render it. Access browser `result.lines` before disposing the result. Once exported, those lines remain usable after disposal.
