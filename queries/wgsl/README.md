# wgsl queries

Adapted from [the grammar’s nvim-compatible queries](https://github.com/webgpu-tools/tree-sitter-wesl/tree/b600a65818e4dd7f4d32407d1427643c27f3c389/queries/nvim).

The WGSL/WESL parser supports current WGSL syntax and keeps comments visible for
highlighting. Its upstream query names differ from nvim-treesitter’s older WGSL
grammar, so its own nvim-compatible queries are used. The `wesl` alias enables the
same parser for WESL sources.

## Rainbow brackets

`rainbows.scm` follows [Helix’s rainbow query model](https://github.com/helix-editor/helix/blob/ba40e547426b0f9896c8bdc699a4ab11f2b37dbc/book/src/guides/rainbow_bracket_queries.md), using the [pinned grammar](https://github.com/webgpu-tools/tree-sitter-wesl/tree/b600a65818e4dd7f4d32407d1427643c27f3c389/.).
