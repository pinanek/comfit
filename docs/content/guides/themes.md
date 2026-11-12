---
title: "Themes"
description: "Choose Comfit themes and customize token colors, rainbow brackets, and line emphasis."
---

# Themes

Themes map syntax captures, such as `keyword` or `function.call`, to colors and font styles. How you provide a theme depends on the package and [HTML style mode](html.md).

Node.js, Python, and the CLI accept bundled theme IDs. Rust selects bundled themes through Cargo features and also supports TOML themes. In the browser, use a stylesheet for class output, a theme object for inline output, or your own CSS variables.

## Choose a bundled theme

Choose an ID from the [bundled theme catalog](#bundled-themes), or compare palettes in the [playground](/playground). Use the examples below to find or apply a theme.

<CodeTabs label="Theme discovery API">
<CodeTab label="Node.js">

```javascript
import { themes } from "@comfit/node";

console.log(themes());
```

</CodeTab>
<CodeTab label="Browser">

Use an initialized highlighter with JavaScript loaded.

Load the stylesheet in your frontend entry point with a bundler that supports CSS imports, then match its outer class.

```typescript
import "@comfit/css/catppuccin-mocha.css";

const html = highlighter.renderHtml("const answer = 42;", "javascript", {
  preClass: "comfit-catppuccin-mocha",
});
```

</CodeTab>
<CodeTab label="Rust">

Enable a theme feature such as `catppuccin-mocha` on `comfit-themes`. The list contains only enabled themes.

Theme IDs use hyphens, while Rust module names use underscores.

```rust
for theme in comfit_themes::THEMES {
    println!("{}: {}", theme.id, theme.name);
}

let theme = comfit_themes::catppuccin_mocha::theme();
```

</CodeTab>
<CodeTab label="Python">

```python
import comfit

print(comfit.themes())
```

</CodeTab>
<CodeTab label="rehype">

Pass a bundled theme ID in your framework's `rehypePlugins` configuration.

```javascript
import rehypeComfit from "@comfit/rehype";

const rehypePlugins = [
  [rehypeComfit, { theme: "catppuccin-mocha" }],
];
```

</CodeTab>
<CodeTab label="Command line">

```sh
comfit ansi --list-themes
```

</CodeTab>
</CodeTabs>

See [HTML and CSS](html.md#classes) for stylesheet installation and layout.

## Color nested brackets

Highlighting collects bracket depth by default. Visible bracket colors still need a palette: set `rainbow` on a theme, or define `.rainbow-N` classes or `--comfit-rainbow-N` variables.

Class and variable output can cycle bracket indices through a fixed number of colors. When rendering class-based HTML without a supplied theme, set the wrapping count to the palette size in the [catalog](#bundled-themes) so the indices match the available CSS classes.

These examples configure a six-color stylesheet such as Nord. The configuration alone does not load a stylesheet.

<CodeTabs label="Bracket colors API">
<CodeTab label="Node.js">

```javascript
import { RenderHtmlConfig } from "@comfit/node";

const config = new RenderHtmlConfig();
config.rainbowColors = 6;
```

</CodeTab>
<CodeTab label="Browser">

Use an [initialized browser highlighter](../getting-started/browser.md) with JavaScript loaded.

```typescript
const html = highlighter.renderHtml("const answer = (42);", "javascript", {
  style: "class",
  rainbowColors: 6,
  preClass: "comfit-nord",
});
```

</CodeTab>
<CodeTab label="Rust">

Enable `render-html` on `comfit` to use `RenderHtmlConfig`.

```rust
let config = comfit::RenderHtmlConfig {
    rainbow_colors: 6,
    ..Default::default()
};
```

</CodeTab>
<CodeTab label="Python">

```python
import comfit

config = comfit.RenderHtmlConfig(rainbow_colors=6)
```

</CodeTab>
<CodeTab label="rehype">

Add this configuration to your framework's `rehypePlugins`.

```javascript
import rehypeComfit from "@comfit/rehype";

const rehypePlugins = [
  [rehypeComfit, { style: "class", rainbowColors: 6, preClass: "comfit-nord" }],
];
```

</CodeTab>
<CodeTab label="Command line">

```sh
comfit html main.js --theme nord --style class --rainbow-colors 6
```

</CodeTab>
</CodeTabs>

In class and inline modes, a nonempty supplied theme palette takes precedence. Bundled Catppuccin Mocha has no rainbow palette, so enabling bracket collection alone does not color its brackets.

## Emphasize lines

Line numbers are one-based. These examples select line 2.

Pass the configuration to the corresponding highlight or render call.

<CodeTabs label="Line emphasis API">
<CodeTab label="Node.js">

```javascript
import { RenderHtmlConfig } from "@comfit/node";

const config = new RenderHtmlConfig();
config.highlightedLines = [2];
```

</CodeTab>
<CodeTab label="Browser">

Use an [initialized browser highlighter](../getting-started/browser.md) with JavaScript loaded.

```typescript
const html = highlighter.renderHtml(
  "const answer = 42;\nconsole.log(answer);",
  "javascript",
  {
    highlightedLines: [2],
  },
);
```

</CodeTab>
<CodeTab label="Rust">

```rust
let config = comfit::HighlightConfig {
    highlighted_lines: vec![2],
    ..Default::default()
};
```

</CodeTab>
<CodeTab label="Python">

```python
import comfit

config = comfit.RenderHtmlConfig(highlighted_lines=[2])
```

</CodeTab>
<CodeTab label="rehype">

Add this configuration to your framework's `rehypePlugins`.

```javascript
import rehypeComfit from "@comfit/rehype";

const rehypePlugins = [
  [rehypeComfit, { highlightedLines: [2] }],
];
```

</CodeTab>
<CodeTab label="Command line">

```sh
comfit html main.js --theme catppuccin-mocha --highlight-lines 2
```

</CodeTab>
</CodeTabs>

Selecting a line marks it for emphasis without assigning a background color. Supply a theme line-highlight style, a `.line-highlight` CSS rule, or line-highlight variables. For full-width backgrounds, make the rendered line spans block-level.

To change themes without parsing or highlighting again, render an existing [document result](documents.md). To change line emphasis, highlight the document with a new line selection. This reuses cached highlighting when the source and rainbow option are unchanged.

## Bundled themes

Every theme below is available as a bundled theme ID and an `@comfit/css` stylesheet. Follow the [class-based HTML guide](html.md#classes) to load a stylesheet and match its outer class. Use the [playground](/playground) to compare palettes.

Appearance describes the theme's background. A stylesheet applies one palette and does not switch automatically with your site's light or dark mode.

**Rainbow colors** is the size of the bundled bracket palette. None means you need to supply a palette to color brackets.

The bundled themes do not define line-highlight styles. See [Emphasize lines](#emphasize-lines) to make selected lines visible.

The playground supplements themes with fallback bracket and line-emphasis colors. The catalog describes the bundled theme files, without those additions.

| Theme                       | ID                           | Appearance | CSS import                                   | Outer class                         | Rainbow colors |
| --------------------------- | ---------------------------- | ---------- | -------------------------------------------- | ----------------------------------- | -------------- |
| Atom One Dark               | `one-dark`                   | Dark       | `@comfit/css/one-dark.css`                   | `comfit-one-dark`                   | None           |
| Atom One Light              | `one-light`                  | Light      | `@comfit/css/one-light.css`                  | `comfit-one-light`                  | None           |
| Ayu Dark                    | `ayu-dark`                   | Dark       | `@comfit/css/ayu-dark.css`                   | `comfit-ayu-dark`                   | None           |
| Ayu Light                   | `ayu-light`                  | Light      | `@comfit/css/ayu-light.css`                  | `comfit-ayu-light`                  | None           |
| Ayu Mirage                  | `ayu-mirage`                 | Dark       | `@comfit/css/ayu-mirage.css`                 | `comfit-ayu-mirage`                 | None           |
| Catppina Dark               | `catppina-dark`              | Dark       | `@comfit/css/catppina-dark.css`              | `comfit-catppina-dark`              | None           |
| Catppina Light              | `catppina-light`             | Light      | `@comfit/css/catppina-light.css`             | `comfit-catppina-light`             | None           |
| Catppuccin Frappé           | `catppuccin-frappe`          | Dark       | `@comfit/css/catppuccin-frappe.css`          | `comfit-catppuccin-frappe`          | None           |
| Catppuccin Latte            | `catppuccin-latte`           | Light      | `@comfit/css/catppuccin-latte.css`           | `comfit-catppuccin-latte`           | None           |
| Catppuccin Macchiato        | `catppuccin-macchiato`       | Dark       | `@comfit/css/catppuccin-macchiato.css`       | `comfit-catppuccin-macchiato`       | None           |
| Catppuccin Mocha            | `catppuccin-mocha`           | Dark       | `@comfit/css/catppuccin-mocha.css`           | `comfit-catppuccin-mocha`           | None           |
| Dracula                     | `dracula`                    | Dark       | `@comfit/css/dracula.css`                    | `comfit-dracula`                    | None           |
| Dracula Soft                | `dracula-soft`               | Dark       | `@comfit/css/dracula-soft.css`               | `comfit-dracula-soft`               | None           |
| GitHub Dark                 | `github-dark`                | Dark       | `@comfit/css/github-dark.css`                | `comfit-github-dark`                | None           |
| GitHub Dark Colorblind      | `github-dark-colorblind`     | Dark       | `@comfit/css/github-dark-colorblind.css`     | `comfit-github-dark-colorblind`     | None           |
| GitHub Dark Default         | `github-dark-default`        | Dark       | `@comfit/css/github-dark-default.css`        | `comfit-github-dark-default`        | None           |
| GitHub Dark Dimmed          | `github-dark-dimmed`         | Dark       | `@comfit/css/github-dark-dimmed.css`         | `comfit-github-dark-dimmed`         | None           |
| GitHub Dark High Contrast   | `github-dark-high-contrast`  | Dark       | `@comfit/css/github-dark-high-contrast.css`  | `comfit-github-dark-high-contrast`  | None           |
| GitHub Light                | `github-light`               | Light      | `@comfit/css/github-light.css`               | `comfit-github-light`               | None           |
| GitHub Light Colorblind     | `github-light-colorblind`    | Light      | `@comfit/css/github-light-colorblind.css`    | `comfit-github-light-colorblind`    | None           |
| GitHub Light Default        | `github-light-default`       | Light      | `@comfit/css/github-light-default.css`       | `comfit-github-light-default`       | None           |
| GitHub Light High Contrast  | `github-light-high-contrast` | Light      | `@comfit/css/github-light-high-contrast.css` | `comfit-github-light-high-contrast` | None           |
| Night Owl                   | `night-owl`                  | Dark       | `@comfit/css/night-owl.css`                  | `comfit-night-owl`                  | None           |
| Night Owl (No Italics)      | `night-owl-no-italics`       | Dark       | `@comfit/css/night-owl-no-italics.css`       | `comfit-night-owl-no-italics`       | None           |
| Nord                        | `nord`                       | Dark       | `@comfit/css/nord.css`                       | `comfit-nord`                       | 6              |
| One Dark Pro                | `one-dark-pro`               | Dark       | `@comfit/css/one-dark-pro.css`               | `comfit-one-dark-pro`               | 3              |
| One Dark Pro Darker         | `one-dark-pro-darker`        | Dark       | `@comfit/css/one-dark-pro-darker.css`        | `comfit-one-dark-pro-darker`        | 3              |
| One Dark Pro Flat           | `one-dark-pro-flat`          | Dark       | `@comfit/css/one-dark-pro-flat.css`          | `comfit-one-dark-pro-flat`          | 3              |
| One Dark Pro Mix            | `one-dark-pro-mix`           | Dark       | `@comfit/css/one-dark-pro-mix.css`           | `comfit-one-dark-pro-mix`           | 3              |
| One Dark Pro Night Flat     | `one-dark-pro-night-flat`    | Dark       | `@comfit/css/one-dark-pro-night-flat.css`    | `comfit-one-dark-pro-night-flat`    | 3              |
| Rosé Pine                   | `rose-pine`                  | Dark       | `@comfit/css/rose-pine.css`                  | `comfit-rose-pine`                  | 6              |
| Rosé Pine (no italics)      | `rose-pine-no-italics`       | Dark       | `@comfit/css/rose-pine-no-italics.css`       | `comfit-rose-pine-no-italics`       | 6              |
| Rosé Pine Dawn              | `rose-pine-dawn`             | Light      | `@comfit/css/rose-pine-dawn.css`             | `comfit-rose-pine-dawn`             | 6              |
| Rosé Pine Dawn (no italics) | `rose-pine-dawn-no-italics`  | Light      | `@comfit/css/rose-pine-dawn-no-italics.css`  | `comfit-rose-pine-dawn-no-italics`  | 6              |
| Rosé Pine Moon              | `rose-pine-moon`             | Dark       | `@comfit/css/rose-pine-moon.css`             | `comfit-rose-pine-moon`             | 6              |
| Rosé Pine Moon (no italics) | `rose-pine-moon-no-italics`  | Dark       | `@comfit/css/rose-pine-moon-no-italics.css`  | `comfit-rose-pine-moon-no-italics`  | 6              |
| Tokyo Night                 | `tokyo-night`                | Dark       | `@comfit/css/tokyo-night.css`                | `comfit-tokyo-night`                | 6              |
| Tokyo Night Light           | `tokyo-night-light`          | Light      | `@comfit/css/tokyo-night-light.css`          | `comfit-tokyo-night-light`          | 6              |
| Tokyo Night Storm           | `tokyo-night-storm`          | Dark       | `@comfit/css/tokyo-night-storm.css`          | `comfit-tokyo-night-storm`          | 6              |
| Vitesse Black               | `vitesse-black`              | Dark       | `@comfit/css/vitesse-black.css`              | `comfit-vitesse-black`              | 6              |
| Vitesse Dark                | `vitesse-dark`               | Dark       | `@comfit/css/vitesse-dark.css`               | `comfit-vitesse-dark`               | 6              |
| Vitesse Dark Soft           | `vitesse-dark-soft`          | Dark       | `@comfit/css/vitesse-dark-soft.css`          | `comfit-vitesse-dark-soft`          | 6              |
| Vitesse Light               | `vitesse-light`              | Light      | `@comfit/css/vitesse-light.css`              | `comfit-vitesse-light`              | 6              |
| Vitesse Light Soft          | `vitesse-light-soft`         | Light      | `@comfit/css/vitesse-light-soft.css`         | `comfit-vitesse-light-soft`         | 6              |

## Custom colors in the browser

With an initialized highlighter that has Rust loaded:

```typescript
import type { Theme } from "@comfit/wasm";

const theme: Theme = {
  name: "Custom",
  default: { fg: "#cdd6f4", bg: "#1e1e2e" },
  styles: {
    keyword: { fg: "#cba6f7" },
    string: { fg: "#a6e3a1" },
    function: { fg: "#89b4fa" },
  },
  rainbow: ["#f38ba8", "#a6e3a1", "#89b4fa"],
  lineHighlight: { bg: "#313244" },
};

const html = highlighter.renderHtml("fn main() {}", "rust", {
  style: "inline",
  theme,
  highlightedLines: [1],
});
```

The `theme` option belongs to inline mode. Class and variables modes do not accept it in the browser API. Node.js and Python rendering functions accept bundled theme IDs, not this object.

## Load a TOML theme in Rust

Save the theme as `custom.toml`, then load it using the Rust or CLI example below.

```toml title="custom.toml"
name = "Custom"
rainbow = ["#f38ba8", "#a6e3a1", "#89b4fa"]
line_highlight = { bg = "#313244" }

[default]
fg = "#cdd6f4"
bg = "#1e1e2e"

[styles]
keyword = { fg = "#cba6f7" }
string = { fg = "#a6e3a1" }
function = { fg = "#89b4fa" }
```

TOML uses `line_highlight`. Browser theme objects use `lineHighlight`. Dotted capture names such as `"function.call"` must be quoted as TOML keys.

Load the file in Rust or pass it to the CLI:

<CodeTabs label="TOML theme API">
<CodeTab label="Rust">

Enable `theme-toml` on `comfit` to parse the file with `Theme::from_toml`.

```rust
let source = std::fs::read_to_string("custom.toml")?;
let theme = comfit::Theme::from_toml(&source)?;
```

</CodeTab>
<CodeTab label="Command line">

```sh
comfit ansi main.rs --theme-file custom.toml
```

</CodeTab>
</CodeTabs>

## Convert a VS Code theme

```sh
comfit convert vscode-theme.json --report-unmapped -o custom.toml
comfit ansi main.rs --theme-file custom.toml
```

The converter accepts JSON5 and resolves local `include` files relative to each theme file. It does not download parent themes.

Use `--report-unmapped` to report unsupported selectors. Review the converted palette before using it.
