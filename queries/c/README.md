# c queries

Adapted from: [Neovim (nvim-treesitter)](https://github.com/nvim-treesitter/nvim-treesitter/tree/main/runtime/queries/c).

## Adaptations

GNU inline assembly templates inject the `asm` grammar when it is loaded. Only string contents are injected, leaving the surrounding quotes and operand constraints in C. C++ and CUDA inherit these injection queries.

## Rainbow brackets

`rainbows.scm` is adapted from
[Helix's rainbow query model](https://github.com/helix-editor/helix/blob/ba40e547426b0f9896c8bdc699a4ab11f2b37dbc/book/src/guides/rainbow_bracket_queries.md),
using the [language grammar](https://github.com/tree-sitter/tree-sitter-c/blob/b780e47fc780ddc8da13afa35a3f4ed5c157823d/grammar.js).
