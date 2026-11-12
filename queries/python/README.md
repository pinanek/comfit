# python queries

Adapted from: [Neovim (nvim-treesitter)](https://github.com/nvim-treesitter/nvim-treesitter/tree/main/runtime/queries/python).

## Adaptations

Docstring captures are restricted to the first statement of a module, class, or function. Comments may precede it; byte strings and f-strings are not docstrings. Class field definitions match assignment nodes directly because `expression_statement` is an invisible supertype in the pinned grammar.

## Rainbow brackets

`rainbows.scm` is adapted from
[Helix's rainbow query model](https://github.com/helix-editor/helix/blob/ba40e547426b0f9896c8bdc699a4ab11f2b37dbc/book/src/guides/rainbow_bracket_queries.md),
using the [language grammar](https://github.com/tree-sitter/tree-sitter-python/blob/26855eabccb19c6abf499fbc5b8dc7cc9ab8bc64/grammar.js).
