---
title: "Lines and tokens"
description: "Read highlighted lines, tokens, source ranges, and syntax captures from Comfit."
---

# Lines and tokens

Use token output to render code yourself, inspect syntax captures, or send highlighting results to another part of your application. A token contains source text and optional styling information, not HTML or final colors.

For ready-to-display output, use the [HTML renderer](html.md) instead.

## Highlight without rendering

<CodeTabs label="Token output API">
<CodeTab label="Node.js">

```javascript
import { highlight } from "@comfit/node";

const lines = highlight("const answer = 42;\n", "javascript");
for (const line of lines) {
  for (const token of line.tokens) {
    console.log(line.number, token.text, token.highlight, token.rainbow);
  }
}
```

</CodeTab>
<CodeTab label="Browser">

Use an [initialized browser highlighter](../getting-started/browser.md) with JavaScript loaded.

```typescript
const lines = highlighter.highlight("const answer = 42;\n", "javascript");
for (const line of lines) {
  for (const token of line.tokens) {
    console.log(line.number, token.text, token.highlight, token.rainbow);
  }
}
```

</CodeTab>
<CodeTab label="Rust">

Enable the `javascript` feature on `comfit`.

```rust
let mut highlighter = comfit::Highlighter::new()?;
let highlighted = highlighter.highlight(
    "const answer = 42;\n",
    "javascript",
    &comfit::HighlightConfig::default(),
)?;
let lines = &highlighted.lines;
for line in lines {
    for token in &line.tokens {
        println!("{} {:?} {:?} {:?}", line.number, token.text, token.highlight, token.rainbow);
    }
}
```

</CodeTab>
<CodeTab label="Python">

```python
import comfit

lines = comfit.highlight("const answer = 42;\n", "javascript")
for line in lines:
    for token in line.tokens:
        print(line.number, token.text, token.highlight, token.rainbow)
```

</CodeTab>
</CodeTabs>

## Read the result

Each line contains:

| Field         | Meaning                                             |
| ------------- | --------------------------------------------------- |
| `number`      | One-based source line number.                       |
| `highlighted` | Whether you selected this line for emphasis.        |
| `tokens`      | Tokens in source order, without newline separators. |

Each token contains:

| Field       | Meaning                                                         |
| ----------- | --------------------------------------------------------------- |
| `text`      | The original source text for this token.                        |
| `highlight` | A capture name such as `keyword`, `string`, or `function.call`. |
| `rainbow`   | Zero-based bracket depth when collected.                        |

Absent captures and depths are `null` in JavaScript, `None` in Python, and `None` in Rust's optional fields. A token boundary is not necessarily a syntax-node boundary: captures can overlap and be split across lines.

An empty source produces one empty line. A trailing newline produces a final empty line. To reconstruct the source, use the `lines` from the highlighting example:

<CodeTabs label="Source reconstruction API">
<CodeTab label="Node.js / Browser">

```javascript
const source = lines
  .map((line) => line.tokens.map((token) => token.text).join(""))
  .join("\n");
```

</CodeTab>
<CodeTab label="Rust">

```rust
let source = lines
    .iter()
    .map(|line| line.tokens.iter().map(|token| token.text).collect::<String>())
    .collect::<Vec<_>>()
    .join("\n");
```

</CodeTab>
<CodeTab label="Python">

```python
source = "\n".join(
    "".join(token.text for token in line.tokens) for line in lines
)
```

</CodeTab>
</CodeTabs>

## Use captures as style names

Captures describe syntax roles rather than final colors. A theme maps them to styles. Dotted names allow more specific styling, such as `function` and `function.call`.

Tokens with no capture still contain source text and must be rendered. A `rainbow` depth is independent of its final color. The renderer cycles through a palette by depth.

## Render tokens safely

In React, render `token.text` as a text child. In plain DOM code, assign it to `textContent`. Do not concatenate unescaped token text into HTML.

If you do not need custom markup, use Comfit's [HTML renderer](html.md), which already escapes source text.

## Keep results after edits

Node.js, Python, and browser token arrays own their text. Rust's one-shot result borrows the source and highlighter. A [document session](documents.md) returns an owned snapshot that remains usable after later edits.
