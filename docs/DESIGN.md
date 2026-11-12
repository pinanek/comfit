---
name: Comfit
description: Visual conventions for the Comfit documentation site and playground.
colors:
  accent: "#16618b"
  accent-hover: "#00647e"
  bg: "#f7f7f8"
  bg-subtle: "#f1f1f2"
  bg-muted: "#eeeeef"
  bg-hover: "#e3e3e5"
  bg-active: "#d8d8da"
  text: "#303033"
  text-muted: "#5b5b5f"
  text-subtle: "#737378"
  code-number: "#9d9da2"
  code-type: "#765702"
  code-keyword: "#41599c"
  code-property: "#00675f"
  code-constant: "#944600"
  code-escape: "#9c356a"
  code-string: "#186846"
  code-error: "#a72f3a"
  code-parameter: "#984133"
  code-operator: "#116285"
  code-macro: "#974036"
  code-heading: "#485a8c"
  accent-dark: "#9ccfe9"
  accent-hover-dark: "#94d1e5"
  bg-dark: "#19191a"
  bg-subtle-dark: "#141415"
  bg-muted-dark: "#212122"
  bg-hover-dark: "#323233"
  bg-active-dark: "#434344"
  text-dark: "#c3c3c4"
  text-muted-dark: "#b3b3b4"
  text-subtle-dark: "#939394"
  code-number-dark: "#636364"
  code-type-dark: "#e9cc9c"
  code-keyword-dark: "#b5ccff"
  code-property-dark: "#7ecebe"
  code-constant-dark: "#fbc19d"
  code-escape-dark: "#e99cba"
  code-string-dark: "#7fc7a4"
  code-error-dark: "#ff9596"
  code-parameter-dark: "#f1a290"
  code-operator-dark: "#8fc9e6"
  code-macro-dark: "#ffcdc7"
  code-heading-dark: "#c1d0f2"
typography:
  display:
    fontFamily: "Hanken Grotesk, system-ui"
    fontSize: "clamp(2.5rem, 4.1vw, 4.0625rem)"
    fontWeight: 700
    lineHeight: 1.1
    letterSpacing: "-0.025em"
  headline:
    fontFamily: "Hanken Grotesk, system-ui"
    fontSize: "clamp(1.75rem, 1.35rem + 1.5vw, 2.25rem)"
    fontWeight: 700
    lineHeight: "calc(1em + 0.5rem)"
  title:
    fontFamily: "Hanken Grotesk, system-ui"
    fontSize: "1.5rem"
    fontWeight: 700
    lineHeight: "calc(1em + 0.5rem)"
  body:
    fontFamily: "Hanken Grotesk, system-ui"
    fontSize: "1rem"
    fontWeight: 500
    lineHeight: "calc(1em + 0.5rem)"
  nav-label:
    fontFamily: "Hanken Grotesk, system-ui"
    fontSize: "0.875rem"
    fontWeight: 600
    lineHeight: "calc(1em + 0.5rem)"
  field-label:
    fontFamily: "Hanken Grotesk, system-ui"
    fontSize: "0.8125rem"
    fontWeight: 600
    lineHeight: "calc(1em + 0.5rem)"
  field-value:
    fontFamily: "Hanken Grotesk, system-ui"
    fontSize: "0.875rem"
    fontWeight: 500
    lineHeight: "calc(1em + 0.5rem)"
  code:
    fontFamily: "JetBrains Mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace"
    fontSize: "0.875rem"
    fontWeight: 500
    lineHeight: "calc(1em + 0.5rem)"
  code-label:
    fontFamily: "JetBrains Mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace"
    fontSize: "0.75rem"
    fontWeight: 500
    lineHeight: "calc(1em + 0.5rem)"
rounded:
  xs: "0.25rem"
  sm: "0.5rem"
  md: "0.75rem"
  lg: "1rem"
spacing:
  compact: "0.25rem"
  control: "0.5rem"
  panel: "0.75rem"
  pane: "1rem"
  gutter: "1.5rem"
  gutter-wide: "2rem"
  prose-xs: "0.375rem"
  prose-sm: "0.75rem"
  prose-md: "1.25rem"
  prose-lg: "1.75rem"
  prose-xl: "2.75rem"
components:
  site-navigation:
    backgroundColor: "{colors.bg}"
    typography: "{typography.nav-label}"
    padding: "0.5rem 1.5rem"
  site-nav-link:
    textColor: "{colors.text-muted}"
    typography: "{typography.nav-label}"
    rounded: "{rounded.sm}"
    padding: "0.25rem 0.5rem"
  site-nav-link-hover:
    backgroundColor: "{colors.bg-muted}"
    textColor: "{colors.text}"
  theme-button:
    backgroundColor: "transparent"
    textColor: "{colors.text-muted}"
    rounded: "{rounded.sm}"
    size: "2rem"
    padding: "{spacing.compact}"
  theme-button-hover:
    backgroundColor: "{colors.bg-muted}"
    textColor: "{colors.text}"
  theme-button-active:
    backgroundColor: "{colors.bg-active}"
  copy-button:
    backgroundColor: "{colors.bg}"
    textColor: "{colors.text-muted}"
    rounded: "{rounded.sm}"
    size: "2.75rem"
  copy-button-hover:
    backgroundColor: "{colors.bg-muted}"
    textColor: "{colors.text}"
  copy-button-active:
    backgroundColor: "{colors.bg-hover}"
  select-field:
    backgroundColor: "{colors.bg}"
    textColor: "{colors.text}"
    typography: "{typography.field-value}"
    rounded: "{rounded.sm}"
    padding: "0.5rem 0.75rem"
  checkbox:
    backgroundColor: "{colors.bg}"
    rounded: "{rounded.xs}"
    size: "1.125rem"
  checkbox-checked:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.bg}"
  code-panel:
    backgroundColor: "{colors.bg}"
    typography: "{typography.code}"
    rounded: "{rounded.md}"
  code-panel-caption:
    backgroundColor: "{colors.bg-hover}"
    textColor: "{colors.text-muted}"
    typography: "{typography.code-label}"
    padding: "0.375rem 1rem"
  code-tab:
    textColor: "{colors.text-muted}"
    typography: "{typography.code-label}"
    padding: "0.375rem 0.75rem"
  code-tab-active:
    textColor: "{colors.accent}"
  note:
    backgroundColor: "color-mix(in oklab, #16618b 10%, #f7f7f8)"
    rounded: "{rounded.md}"
    padding: "1rem 1.5rem"
  note-dark:
    backgroundColor: "color-mix(in oklab, #9ccfe9 10%, #19191a)"
---

# Design System: Comfit

## Overview

Comfit pairs a neutral page with code that carries most of the color. The same light and dark palettes carry through the site header, documentation, playground, and examples. Hanken Grotesk provides the reading hierarchy, and JetBrains Mono identifies source, output, and code metadata.

Thin rules and adjacent surface tones distinguish navigation, controls, and code panels. Underlined links, accent outlines, and selected states show where readers can act and where they are.

**Key Characteristics**

- Light and dark share the same semantic color assignments.
- Code samples and interactive previews use monospaced text and distinct syntax colors.
- Compact controls and a narrow reading column keep dense API material readable.

## Colors

### Primary

Blue carries links, focus rings, the current documentation location, and checked controls. Sapphire provides the alternate link hover color. In code, the same pair marks functions and labels.

### Secondary

Syntax colors represent specific roles. Indigo marks keywords, gold marks types, teal marks properties, green marks strings, and copper marks constants. Comments use subtle ink and italics. The alert palette reuses the same families for note, tip, important, warning, and caution labels, with a 10% tint of the relevant accent over the canvas.

### Neutral

The light canvas is a cool off-white with charcoal ink. The dark canvas is near-black with soft gray ink. Muted ink carries supporting copy, inactive navigation, and panel labels. Progressively stronger neutral tones mark inset surfaces, hovered and pressed controls, and borders. The normal border shares the raised tone and the stronger border shares the pressed tone.

**The Mode Pair Rule.** Use the semantic palette aliases so links, surfaces, and syntax respond to the site theme together.

The palette and its semantic aliases are defined in `src/styles/theme/palette.css` and `src/styles/theme/vars.css`. Dark token names in the frontmatter denote the alternate values of those same aliases.

## Typography

Hanken Grotesk with a system UI fallback carries headings, body text, and interface labels. Body text uses weight 500. Navigation links and field labels use 600. Documentation headings and the wordmark use 700. Reference headings step from a fluid page heading to 1.5rem, 1.25rem, and 1rem subheadings.

JetBrains Mono with system monospace fallbacks carries code, keyboard commands, and code panel captions and tabs. Reference code is 0.875rem and its captions and line numbers are 0.75rem. The base leading adds 0.5rem to the current font size.

## Layout

The site header, documentation shell, and playground use a 90rem maximum width. Main gutters step from 1.5rem to 2rem at 40rem. Documentation prose uses its own vertical spacing scale, with tighter gaps after headings and more space around major sections, tables, and figures.

- Below 40rem, the documentation navigation uses a sticky menu and side drawer.
- At 40rem, a 15rem sticky sidebar sits alongside the reading column.
- At 60rem, the playground editor and output change from stacked to two equal columns.
- At 72em, the table of contents moves from its inline disclosure into a sticky 13rem rail where the page has one.

**The Reading Width Rule.** Keep reference prose in the 72ch column and scroll wide tables and code inside their own containers.

## Elevation & Depth

The shared surfaces are flat. Thin rules separate the header and frame panels, and neutral fills distinguish headers, gutters, hover states, and active lines. The mobile docs drawer sits above a tinted backdrop. Inset strokes indicate selected code tabs. The navigation progress bar uses a small accent glow.

## Shapes

The four shared corner radii range from 0.25rem on inline code, checkboxes, and compact controls to 1rem on documentation images. The middle steps are 0.5rem for inputs and compact buttons and 0.75rem for code panels, tables, and callouts. Code panels clip to their rounded shells while their code scrolls horizontally.

## Components

### Site navigation

The shared header has a wordmark and muted text links, with GitHub and the theme toggle grouped at the trailing edge. Items can wrap on small screens. Links and the theme button take on a muted fill and stronger ink on hover. The docs sidebar adds an accent tint and bold accent text for the current page.

### Buttons and fields

The theme toggle is a compact 2rem icon button. The code copy button is a bordered 2.75rem square, with green ink for success and red for error. Playground selects pair a neutral surface and strong border with a compact sans-serif label. A selected checkbox uses a solid accent fill and a checkmark.

**The Visible Focus Rule.** Use the 2px accent outline for keyboard focus. Inset it inside clipped code and playground controls.

### Code panels

A 1px border encloses a tonal caption bar, a horizontal scroller, and JetBrains Mono code with a muted line-number gutter. A filename or caption can be paired with a language label. Selected language tabs use accent ink and a 2px inset underline. Highlighted lines use the muted surface.

### Alerts and tables

Documentation alerts pair an explicit severity label with a light color tint. Tables use a muted header, horizontal row separators, tabular numerals, and a rounded frame that scrolls independently when needed.
