# @comfit/markdown-it

Comfit plugin for highlighting fenced code blocks with markdown-it, backed by [@comfit/node](../node/README.md).

[Documentation](https://comfit.pinanek.me/docs/guides/markdown#use-with-markdown-it) · [npm](https://www.npmjs.com/package/@comfit/markdown-it)

## Installation

Requires Node.js 22 or later and markdown-it 14 or 15.

```sh
npm install @comfit/markdown-it markdown-it
```

## Usage

````javascript
import markdownItComfit from "@comfit/markdown-it";
import MarkdownIt from "markdown-it";

const md = new MarkdownIt().use(markdownItComfit, {
  theme: "catppuccin-mocha",
});

console.log(md.render("```rust\nfn main() {}\n```\n"));
````

The plugin renders inline theme colors when `theme` or `themes` is supplied. Without a theme, it emits capture classes for a caller-provided stylesheet. See the [HTML and CSS guide](https://comfit.pinanek.me/docs/guides/html#classes) for stylesheets and layout.

## Options

| Option             | Default                                      | Purpose                                                                                                                                            |
| ------------------ | -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `theme`            | None                                         | Bundled theme ID for a single palette.                                                                                                             |
| `themes`           | None                                         | Named bundled theme IDs, such as `{ light: "catppina-light", dark: "catppina-dark" }`. Requires inline output and cannot be combined with `theme`. |
| `defaultTheme`     | First name in `themes`                       | Select the initial named theme.                                                                                                                    |
| `style`            | `"inline"` with a theme, otherwise `"class"` | Emit inline colors, capture classes, or CSS variables with `"variables"`.                                                                          |
| `defaultLanguage`  | None                                         | Language for unlabelled fences.                                                                                                                    |
| `aliases`          | None                                         | Additional language aliases, resolved before built-in IDs and aliases.                                                                             |
| `unknownLanguage`  | `"skip"`                                     | Use the previous fence renderer for unknown languages, or throw with `"error"`.                                                                    |
| `rainbow`          | `true`                                       | Collect bracket nesting depths.                                                                                                                    |
| `rainbowColors`    | `0`                                          | Wrap bracket classes to a fixed palette size. Zero preserves depth unless a theme palette applies.                                                 |
| `classPrefix`      | `""`                                         | Prefix generated token, rainbow, and line classes.                                                                                                 |
| `preClass`         | None                                         | Add a class to the outer `pre` element.                                                                                                            |
| `highlightedLines` | `[]`                                         | Emphasize one-based source lines in every fence.                                                                                                   |

Language classes respect markdown-it's `langPrefix` setting. Token attributes from earlier plugins are preserved without modifying the tokens.

Inline code, indented code blocks, and unlabelled fences without `defaultLanguage` use markdown-it's existing rendering. Fence metadata after the language name is ignored. The plugin does not sanitize other Markdown output.

## License

[MIT](LICENSE). Native parser and query attribution is distributed with `@comfit/node`.
