---
title: "Command line"
description: "Install Comfit from GitHub Releases or Cargo and highlight files in your terminal or as HTML."
---

# Command line

The `comfit` command highlights files or standard input as terminal text or HTML. It also converts VS Code themes to Comfit TOML.

## Install

### Download a prebuilt binary

Open the [latest CLI release](https://github.com/pinanek/comfit/releases/latest) and choose an archive for your operating system and CPU architecture. Extract it and place `comfit` (`comfit.exe` on Windows) in a directory on your `PATH`.

Check the installation:

```sh
comfit --help
```

If no binary matches your platform, build with Cargo instead.

### Build with Cargo

```sh
cargo install comfit-cli
```

The package is named `comfit-cli` and installs the `comfit` command. Building requires Rust and C/C++ build tools. For a repository checkout, follow [Build from source](../development/build-from-source.md#command-line).

## Highlight a file

```sh
comfit ansi main.rs --theme catppuccin-mocha --line-numbers
```

This prints highlighted Rust source with line numbers. Comfit infers the language from the file extension. Use `--language` to override it.

Select a bundled theme with `--theme ID` or load a Comfit TOML theme with `--theme-file PATH`.

List bundled languages and themes:

```sh
comfit ansi --list-languages
comfit ansi --list-themes
```

## Read standard input

```sh
printf 'fn main() {}\n' | comfit ansi --language rust --theme catppuccin-mocha
```

Omit the filename or use `-` to read stdin. Always supply `--language` when reading stdin.

## Save HTML

```sh
comfit html main.rs --theme catppuccin-mocha -o code.html
```

The result is an HTML fragment with inline colors. For a shared stylesheet, add `--style class` and match its outer class:

```sh
comfit html main.rs --theme catppuccin-mocha --style class \
  --pre-class comfit-catppuccin-mocha -o code.html
```

Load the [matching CSS theme](../guides/html.md#classes) on the page that displays the fragment. See [HTML and CSS](../guides/html.md) for CSS variables and layout.

## Wrap and select lines

```sh
comfit ansi main.rs --theme catppuccin-mocha -n --width 100
comfit ansi main.rs --theme catppuccin-mocha --wrap word --line-range 20:60
comfit ansi main.rs --theme catppuccin-mocha --highlight-lines 2,4-6
```

`--width` includes the line-number gutter and enables character wrapping. `--wrap word` prefers word boundaries. `--wrap never` disables wrapping.

Use `--line-range` to show only part of a file. Use `--highlight-lines` to emphasize lines without hiding the rest.

## Pipe colored output

Redirected output is plain text by default. Keep ANSI colors when sending output to a pager:

```sh
comfit ansi main.rs --theme catppuccin-mocha --color always | less -R
```

`NO_COLOR` and `TERM=dumb` disable automatic color. `--color always` overrides them.

For theme conversion, see [Convert a VS Code theme](../guides/themes.md#convert-a-vs-code-theme). Run `comfit ansi --help` or `comfit html --help` for all flags.
