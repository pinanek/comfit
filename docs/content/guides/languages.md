---
title: "Languages"
description: "Find supported languages, choose which grammars to load, and check their embedded-language dependencies."
---

# Languages

Comfit identifies languages by an ID such as `javascript` or an alias such as `js`. Each language has a Tree-sitter grammar for parsing source and queries that assign syntax capture names. Themes turn those captures into token styles.

Use this guide to find a language, include it in your application, and check which embedded languages it can request. For installation and a first render, start with your environment's [quickstart](../index.md#choose-your-environment).

## Find a language

The [language catalog](#language-catalog) lists all registered languages, including helper grammars used inside other languages. Use an ID or one of its aliases when highlighting. A display name or file extension is not necessarily an accepted language name.

To list the languages available in a bundled package:

<CodeTabs>
<CodeTab label="Node.js">

```javascript
import { languages } from "@comfit/node";

console.log(languages());
```

</CodeTab>
<CodeTab label="Python">

```python
import comfit

print(comfit.languages())
```

</CodeTab>
<CodeTab label="Command line">

```sh
comfit ansi --list-languages
```

</CodeTab>
</CodeTabs>

The CLI can infer a language from a filename. In library calls, pass the language ID or alias explicitly.

## Choose which languages to include

Node.js, Python, rehype, and the CLI bundle the registered languages. Rust applications select languages at build time through Cargo features. Browser applications load the parser assets they need when creating a highlighter.

### Select Rust grammars

Enable language features on `comfit`:

```sh
cargo add comfit --features rust,javascript,render-html
```

This includes Rust and JavaScript with HTML rendering. No languages are enabled by default.

Use `all-languages` to include every bundled language. Features generally use the IDs in the catalog, including underscores in names such as `c_sharp`.

Enable embedded languages too. The `markdown` feature also enables `markdown_inline`, but a Rust code fence still needs the `rust` feature. See the [Rust quickstart](../getting-started/rust.md) for a complete rendering example.

## Load browser languages

Follow the [browser quickstart](../getting-started/browser.md#install) to install `@comfit/wasm` and `@comfit/wasm-grammars` and [initialize both runtimes](../getting-started/browser.md#highlight-a-snippet). Then load a language descriptor and its parser WASM file:

```typescript
import { createHighlighter } from "@comfit/wasm";
import rust from "@comfit/wasm-grammars/rust";
import rustWasm from "@comfit/wasm-grammars/tree-sitter-rust.wasm?url";

const highlighter = await createHighlighter({
  languages: [{ ...rust, wasm: rustWasm }],
});

try {
  console.log(highlighter.highlight("fn main() {}", "rust"));
} finally {
  highlighter.dispose();
}
```

The descriptor contains the language ID, aliases, and queries. The WASM file contains the parser.

Importing the descriptor alone does not fetch the parser. `createHighlighter` loads the asset and registers the language.

This example uses Vite's `?url` imports. With another build tool, serve the parser as a public asset and set `wasm` to its URL. Keep `@comfit/wasm` and `@comfit/wasm-grammars` versions matched.

Descriptor import paths use hyphens where language IDs use underscores. For example, import `@comfit/wasm-grammars/markdown-inline` for the `markdown_inline` language.

Parser filenames use the grammar's name, which can differ from the language ID. JSX shares JavaScript's parser, for example. Use the descriptor's asset reference or explicitly supply the correct parser URL.

## Include embedded languages

An injection is a region of source highlighted with another grammar. Markdown with Rust code fences needs `markdown`, `markdown_inline`, and `rust`.

In the browser, pass all three descriptors to `createHighlighter`. In Rust, enable `markdown` and `rust`.

An unloaded root language is an error. An unloaded embedded language is skipped, leaving the outer language's styling in place. A fence label or alias does not load a parser automatically.

The catalog's injection column shows direct targets, not the full dependency chain. Follow those targets if you want their embedded content highlighted too. For example, JavaScript can request HTML, which can in turn request CSS or another script language.

See [Embedded languages](embedded-languages.md) for a complete browser registration example and injection-query behavior.

## Use parsers and queries directly

Most applications only need `comfit` or `@comfit/wasm`. Use `comfit-grammars` when building a Rust integration that needs parser entry points and query strings:

```sh
cargo add comfit-grammars --features rust
```

The `rust` module exposes `LANGUAGE`, `HIGHLIGHTS_QUERY`, `INJECTIONS_QUERY`, `LOCALS_QUERY`, and `RAINBOWS_QUERY`. See the [grammar crate reference](https://docs.rs/comfit-grammars/latest/comfit_grammars/) for Rust and the [Language descriptor reference](../api-references/javascript/wasm-grammars/interfaces/Language.md) for browser integrations.

## Language catalog

The catalog follows `registry/languages.toml` and the resolved bundled injection queries, including inherited queries. File associations list registered extensions and filenames. Languages without file associations are often helper grammars used through injections.

The **Injection languages** column uses language IDs:

- Named targets are explicitly selected by a query or by a constrained language capture.
- **Source-selected** means a query reads a language name from source, such as a code-fence label, heredoc delimiter, template tag, or `lang` attribute. The query may restrict which names it accepts. Only registered, loaded targets can be highlighted.
- **Parent language** means an injected region requests the language that contains its current layer. A language may also inject itself.
- **None** means the bundled query has no direct injection targets. It does not describe languages that might inject this one.

These are query rules, not a promise that every string or comment matching a language name will be treated as embedded code. Custom queries can change the targets.

| Language           | ID                   | Aliases                                | File extensions / names                              | Injection languages                                                                                                             |
| ------------------ | -------------------- | -------------------------------------- | ---------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Angular            | `angular`            | None                                   | None                                                 | `comment`, `css`, `javascript`, `json`, `regex`, Source-selected                                                                |
| Assembly           | `asm`                | `assembly`, `gas`                      | `s`                                                  | `comment`                                                                                                                       |
| Astro              | `astro`              | None                                   | `astro`                                              | `comment`, `css`, `javascript`, `json`, `regex`, `scss`, `typescript`, Source-selected                                          |
| Bash               | `bash`               | `sh`, `shell`                          | `sh`, `bash`                                         | `bash`, `comment`, `printf`, `readline`, `regex`, Source-selected                                                               |
| BibTeX             | `bibtex`             | `bib`                                  | `bib`                                                | `comment`                                                                                                                       |
| C                  | `c`                  | None                                   | `c`, `h`                                             | `asm`, `c`, `comment`, `doxygen`, `printf`, `re2c`                                                                              |
| C#                 | `c_sharp`            | `csharp`, `cs`, `c#`                   | `cs`                                                 | `comment`                                                                                                                       |
| C++                | `cpp`                | `c++`, `cc`, `cxx`                     | `cpp`, `cc`, `cxx`, `hpp`, `hh`, `hxx`               | `asm`, `comment`, `cpp`, `doxygen`, `printf`, `re2c`, Source-selected                                                           |
| C3                 | `c3`                 | None                                   | `c3`, `c3i`, `c3t`                                   | `comment`                                                                                                                       |
| CMake              | `cmake`              | None                                   | `cmake`, `CMakeLists.txt`                            | `comment`                                                                                                                       |
| Comment            | `comment`            | None                                   | None                                                 | None                                                                                                                            |
| CSS                | `css`                | None                                   | `css`                                                | `comment`                                                                                                                       |
| CSV                | `csv`                | None                                   | `csv`                                                | None                                                                                                                            |
| CUDA               | `cuda`               | None                                   | `cu`, `cuh`                                          | `asm`, `comment`, `cuda`, `doxygen`, `printf`, `re2c`, Source-selected                                                          |
| Dart               | `dart`               | None                                   | `dart`                                               | `comment`                                                                                                                       |
| Diff               | `diff`               | `patch`                                | `diff`, `patch`                                      | `comment`                                                                                                                       |
| Dockerfile         | `dockerfile`         | `containerfile`, `docker`, `container` | `dockerfile`, `containerfile`, `docker`, `container` | `bash`, `comment`, `json`, `toml`, `xml`, `yaml`                                                                                |
| Doxygen            | `doxygen`            | None                                   | None                                                 | `html`, Source-selected, Parent language                                                                                        |
| DTD                | `dtd`                | None                                   | `dtd`                                                | `comment`                                                                                                                       |
| EEx                | `eex`                | None                                   | `eex`                                                | `elixir`, `html`                                                                                                                |
| Elixir             | `elixir`             | `ex`                                   | `ex`, `exs`                                          | `comment`, `eex`, `heex`, `json`, `markdown`, `regex`, `surface`, `zig`                                                         |
| Erlang             | `erlang`             | `erl`                                  | `erl`, `hrl`                                         | `comment`                                                                                                                       |
| F#                 | `fsharp`             | `f#`, `fs`                             | `fs`, `fsx`                                          | `comment`, `xml`                                                                                                                |
| Fish               | `fish`               | None                                   | `fish`                                               | `comment`                                                                                                                       |
| Fortran            | `fortran`            | None                                   | `f`, `for`, `f90`, `f95`, `f03`, `f08`, `f18`, `fpp` | `comment`                                                                                                                       |
| GDScript           | `gdscript`           | None                                   | `gd`                                                 | `comment`                                                                                                                       |
| Git Attributes     | `gitattributes`      | None                                   | `gitattributes`, `.gitattributes`                    | `comment`                                                                                                                       |
| Git Config         | `git_config`         | `gitconfig`                            | `gitconfig`, `.gitconfig`                            | `bash`, `comment`                                                                                                               |
| Gleam              | `gleam`              | None                                   | `gleam`                                              | `comment`                                                                                                                       |
| Glimmer            | `glimmer`            | `hbs`, `handlebars`                    | `hbs`                                                | `comment`, `css`, `javascript`                                                                                                  |
| Go                 | `go`                 | `golang`                               | `go`                                                 | `comment`, `printf`, `re2c`, `regex`                                                                                            |
| GraphQL            | `graphql`            | `gql`                                  | `graphql`, `gql`                                     | `comment`                                                                                                                       |
| GROQ               | `groq`               | None                                   | `groq`                                               | `comment`                                                                                                                       |
| Haskell            | `haskell`            | `hs`                                   | `hs`                                                 | `comment`, `css`, `graphql`, `haskell_persistent`, `html`, `javascript`, `json`, `python`, `sql`, `typescript`, Source-selected |
| Haskell Persistent | `haskell_persistent` | None                                   | None                                                 | None                                                                                                                            |
| HCL                | `hcl`                | None                                   | `hcl`                                                | `comment`, Source-selected                                                                                                      |
| HEEx               | `heex`               | None                                   | `heex`                                               | `comment`, `elixir`                                                                                                             |
| HTML               | `html`               | `htm`                                  | `html`, `htm`, `xhtml`                               | `comment`, `css`, `javascript`, `json`, `python`, `regex`, `toml`, Source-selected                                              |
| HTTP               | `http`               | None                                   | `http`, `rest`                                       | `comment`, `graphql`, `javascript`, `json`, `xml`, Source-selected                                                              |
| INI                | `ini`                | `dosini`                               | `ini`, `cfg`                                         | `comment`                                                                                                                       |
| Java               | `java`               | None                                   | `java`                                               | `comment`, `javadoc`, `printf`                                                                                                  |
| Javadoc            | `javadoc`            | None                                   | None                                                 | `html`, `markdown_inline`, `printf`, Source-selected                                                                            |
| JavaScript         | `javascript`         | `js`, `ecmascript`, `x-javascript`     | `js`, `mjs`, `cjs`                                   | `angular`, `comment`, `css`, `glimmer`, `graphql`, `groq`, `html`, `jsdoc`, `regex`, `sql`, `styled`, Source-selected           |
| Jinja              | `jinja`              | `jinja2`, `j2`                         | `jinja`, `jinja2`, `j2`                              | `comment`, `jinja_inline`                                                                                                       |
| Jinja Inline       | `jinja_inline`       | None                                   | None                                                 | `comment`                                                                                                                       |
| JSDoc              | `jsdoc`              | None                                   | None                                                 | None                                                                                                                            |
| JSON               | `json`               | `jsonc`, `jsonld`, `ld+json`           | `json`, `jsonc`, `jsonld`                            | `comment`                                                                                                                       |
| JSON5              | `json5`              | None                                   | `json5`                                              | `comment`                                                                                                                       |
| JSX                | `jsx`                | None                                   | `jsx`                                                | `angular`, `comment`, `css`, `glimmer`, `graphql`, `groq`, `html`, `jsdoc`, `regex`, `sql`, `styled`, Source-selected           |
| Julia              | `julia`              | `jl`                                   | `jl`                                                 | `bash`, `comment`, `markdown`, `regex`                                                                                          |
| Just               | `just`               | `justfile`                             | `just`, `justfile`, `.justfile`                      | `bash`, `comment`, `javascript`, `python`, `regex`, Source-selected                                                             |
| KDL                | `kdl`                | None                                   | `kdl`                                                | `comment`                                                                                                                       |
| Kotlin             | `kotlin`             | `kt`, `kts`                            | `kt`, `kts`                                          | `comment`, `printf`, `regex`                                                                                                    |
| LaTeX              | `latex`              | `tex`                                  | `tex`, `sty`, `cls`, `ltx`                           | `c`, `comment`, `lua`, `python`, Source-selected                                                                                |
| Lua                | `lua`                | None                                   | `lua`                                                | `c`, `comment`, `lua`, `luadoc`, `luap`, `printf`, `query`, `vim`                                                               |
| Lua Pattern        | `luap`               | None                                   | None                                                 | None                                                                                                                            |
| LuaDoc             | `luadoc`             | None                                   | None                                                 | None                                                                                                                            |
| Make               | `make`               | `makefile`                             | `mk`, `mak`, `Makefile`, `GNUmakefile`               | `bash`, `comment`                                                                                                               |
| Markdown           | `markdown`           | `md`                                   | `md`, `markdown`                                     | `html`, `markdown_inline`, `toml`, `yaml`, Source-selected                                                                      |
| Markdown Inline    | `markdown_inline`    | None                                   | None                                                 | None                                                                                                                            |
| NASM               | `nasm`               | None                                   | `asm`, `nasm`                                        | `comment`                                                                                                                       |
| Nix                | `nix`                | None                                   | `nix`                                                | `bash`, `comment`, `fish`, `haskell`, `javascript`, `lua`, `perl`, `python`, `regex`, `rust`, Source-selected                   |
| Nushell            | `nu`                 | `nushell`                              | `nu`                                                 | `comment`                                                                                                                       |
| OCaml              | `ocaml`              | None                                   | `ml`                                                 | `comment`                                                                                                                       |
| OCaml Interface    | `ocaml_interface`    | None                                   | `mli`                                                | `comment`                                                                                                                       |
| Odin               | `odin`               | None                                   | `odin`                                               | `comment`                                                                                                                       |
| Passwd             | `passwd`             | None                                   | `passwd`                                             | None                                                                                                                            |
| Perl               | `perl`               | None                                   | `pl`, `pm`                                           | `comment`, `perl`, `pod`, Source-selected                                                                                       |
| PHP                | `php`                | None                                   | `php`, `phtml`                                       | `bash`, `html`, `phpdoc`, `regex`, Source-selected                                                                              |
| PHPDoc             | `phpdoc`             | None                                   | None                                                 | None                                                                                                                            |
| POD                | `pod`                | None                                   | `pod`                                                | None                                                                                                                            |
| PowerShell         | `powershell`         | `ps1`, `pwsh`                          | `ps1`, `psm1`, `psd1`                                | `comment`, `powershell`, `regex`                                                                                                |
| Printf             | `printf`             | None                                   | None                                                 | None                                                                                                                            |
| Prisma             | `prisma`             | None                                   | `prisma`                                             | `comment`                                                                                                                       |
| PromQL             | `promql`             | None                                   | `promql`                                             | `comment`, `regex`                                                                                                              |
| Protocol Buffers   | `proto`              | `protobuf`                             | `proto`                                              | `comment`                                                                                                                       |
| Pug                | `pug`                | None                                   | `pug`                                                | `comment`, `javascript`, Source-selected                                                                                        |
| Python             | `python`             | `py`                                   | `py`, `pyi`, `pyw`                                   | `comment`, `printf`, `regex`                                                                                                    |
| R                  | `r`                  | `rscript`                              | `r`, `rprofile`, `.Rprofile`                         | `comment`                                                                                                                       |
| RBS                | `rbs`                | None                                   | `rbs`                                                | `comment`                                                                                                                       |
| re2c               | `re2c`               | None                                   | `re2c`                                               | `comment`, Parent language                                                                                                      |
| Readline           | `readline`           | None                                   | `inputrc`                                            | `comment`                                                                                                                       |
| Regex              | `regex`              | `regexp`                               | None                                                 | None                                                                                                                            |
| Ruby               | `ruby`               | `rb`                                   | `rb`, `rake`, `gemspec`                              | `comment`, `rbs`, `regex`, `ruby`, Source-selected                                                                              |
| Rust               | `rust`               | `rs`                                   | `rs`                                                 | `comment`, `html`, `json`, `re2c`, `regex`, `rust`, `slint`, `xml`, Source-selected                                             |
| Scala              | `scala`              | None                                   | `scala`, `sc`                                        | `comment`                                                                                                                       |
| SCSS               | `scss`               | None                                   | `scss`                                               | `comment`                                                                                                                       |
| Slint              | `slint`              | None                                   | `slint`                                              | `comment`                                                                                                                       |
| SQL                | `sql`                | None                                   | `sql`                                                | `comment`                                                                                                                       |
| Styled Components  | `styled`             | None                                   | None                                                 | `comment`, `javascript`                                                                                                         |
| Surface            | `surface`            | None                                   | `sface`                                              | `comment`, `elixir`                                                                                                             |
| Svelte             | `svelte`             | None                                   | `svelte`                                             | `comment`, `css`, `javascript`, `json`, `pug`, `regex`, `scss`, `typescript`, Source-selected                                   |
| Swift              | `swift`              | None                                   | `swift`                                              | `comment`, `regex`                                                                                                              |
| Terraform          | `terraform`          | `tf`, `opentofu`                       | `tf`, `tfvars`, `tofu`                               | `comment`, Source-selected                                                                                                      |
| TOML               | `toml`               | None                                   | `toml`                                               | `comment`                                                                                                                       |
| Tree-sitter Query  | `query`              | `tsquery`                              | `scm`                                                | `comment`, `luap`, `regex`                                                                                                      |
| TSX                | `tsx`                | None                                   | `tsx`                                                | `angular`, `comment`, `css`, `glimmer`, `graphql`, `groq`, `html`, `jsdoc`, `regex`, `sql`, `styled`, Source-selected           |
| TypeScript         | `typescript`         | `ts`, `x-typescript`                   | `ts`, `mts`, `cts`                                   | `angular`, `comment`, `css`, `glimmer`, `graphql`, `groq`, `html`, `jsdoc`, `regex`, `sql`, `styled`, Source-selected           |
| Typst              | `typst`              | `typ`                                  | `typ`                                                | `comment`, Source-selected                                                                                                      |
| Vim Script         | `vim`                | `vimscript`                            | `vim`, `vimrc`                                       | `comment`, `lua`, `perl`, `python`, `regex`, `ruby`, `vim`                                                                      |
| Vue                | `vue`                | None                                   | `vue`                                                | `comment`, `css`, `javascript`, `json`, `jsx`, `pug`, `regex`, `scss`, `tsx`, `typescript`, Source-selected                     |
| WGSL               | `wgsl`               | `wesl`                                 | `wgsl`, `wesl`                                       | `comment`                                                                                                                       |
| XML                | `xml`                | None                                   | `xml`, `xsd`, `xsl`, `xslt`, `svg`                   | `comment`, `css`, `javascript`, `sql`                                                                                           |
| YAML               | `yaml`               | `yml`                                  | `yaml`, `yml`                                        | `bash`, `comment`, `promql`                                                                                                     |
| Zig                | `zig`                | None                                   | `zig`, `zon`                                         | `comment`                                                                                                                       |
