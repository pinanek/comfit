# vim queries

Adapted from [Neovim (nvim-treesitter)](https://github.com/nvim-treesitter/nvim-treesitter/tree/915f2bd3d4e2a33704bfc476b5137e9befa82f3d/runtime/queries/vim).

## Adaptations

Inline and heredoc Perl code injects the `perl` grammar when it is loaded, alongside the existing Lua, Ruby, and Python injections. Script heredoc modifiers such as `trim` are highlighted as keywords.

## Rainbow brackets

`rainbows.scm` follows [Helix's rainbow query model](https://github.com/helix-editor/helix/blob/ba40e547426b0f9896c8bdc699a4ab11f2b37dbc/book/src/guides/rainbow_bracket_queries.md), using the [pinned grammar](https://github.com/tree-sitter-grammars/tree-sitter-vim/tree/039c8d0aa1deae00ddeb0374dd70bcc0ec56938d/.).
