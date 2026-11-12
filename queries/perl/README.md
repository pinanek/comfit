# perl queries

Adapted from [Neovim (nvim-treesitter)](https://github.com/nvim-treesitter/nvim-treesitter/tree/915f2bd3d4e2a33704bfc476b5137e9befa82f3d/runtime/queries/perl).

## Adaptations

Generic function captures precede call and builtin captures so they do not override the more specific styles.

## Rainbow brackets

`rainbows.scm` follows [Helix's rainbow query model](https://github.com/helix-editor/helix/blob/ba40e547426b0f9896c8bdc699a4ab11f2b37dbc/book/src/guides/rainbow_bracket_queries.md), using the [pinned grammar](https://github.com/tree-sitter-perl/tree-sitter-perl/tree/80781e3dd98108199a3a1b5860f00fac22088b79/.).
