# kotlin queries

Adapted from [Neovim (nvim-treesitter)](https://github.com/nvim-treesitter/nvim-treesitter/tree/19bdfc04f40ed2ccb3340591d125f759b0406875/runtime/queries/kotlin).

Lua text predicates are converted to portable `#match?` expressions; multiline
dots use `[\\s\\S]` to preserve Lua matching behavior.

## Rainbow brackets

`rainbows.scm` follows [Helix’s rainbow query model](https://github.com/helix-editor/helix/blob/ba40e547426b0f9896c8bdc699a4ab11f2b37dbc/book/src/guides/rainbow_bracket_queries.md), using the [pinned grammar](https://github.com/fwcd/tree-sitter-kotlin/tree/d6af96d2e23ef0a8ff5ab512864378cf08fba311/.).
