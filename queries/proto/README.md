# proto queries

Adapted from [Neovim (nvim-treesitter)](https://github.com/nvim-treesitter/nvim-treesitter/tree/19bdfc04f40ed2ccb3340591d125f759b0406875/runtime/queries/proto).

Lua text predicates are converted to portable `#match?` expressions; multiline
dots use `[\\s\\S]` to preserve Lua matching behavior.

## Rainbow brackets

`rainbows.scm` follows [Helix’s rainbow query model](https://github.com/helix-editor/helix/blob/ba40e547426b0f9896c8bdc699a4ab11f2b37dbc/book/src/guides/rainbow_bracket_queries.md), using the [pinned grammar](https://github.com/coder3101/tree-sitter-proto/tree/18366491f37d43d6d5d02cbb1e79b650b5728145/.).
