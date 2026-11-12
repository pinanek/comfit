# jinja queries

Adapted from [Neovim (nvim-treesitter)](https://github.com/nvim-treesitter/nvim-treesitter/tree/19bdfc04f40ed2ccb3340591d125f759b0406875/runtime/queries/jinja).

The pinned upstream is archived. Jinja’s inline line-statement grammar is bundled
as `jinja_inline`, matching nvim-treesitter’s parser split. Template text is left
unstyled; these queries highlight Jinja syntax rather than an embedded host language.

## Rainbow brackets

`rainbows.scm` follows [Helix’s rainbow query model](https://github.com/helix-editor/helix/blob/ba40e547426b0f9896c8bdc699a4ab11f2b37dbc/book/src/guides/rainbow_bracket_queries.md), using the [pinned grammar](https://github.com/cathaysia/tree-sitter-jinja/tree/f46e979278123e3b6949381896f3917f378af068/tree-sitter-jinja).
