# Queries

Highlight, injection, and local queries come from
[Neovim's nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter/tree/main/runtime/queries),
except Dockerfile queries, which come from
[wharflab/tree-sitter-containerfile](https://github.com/wharflab/tree-sitter-containerfile),
and WGSL queries, which come from the grammar’s
[nvim-compatible queries](https://github.com/webgpu-tools/tree-sitter-wesl/tree/main/queries/nvim).
Text matching uses `#match?` with regex syntax supported by Rust and JavaScript.
Lua character classes are expanded to ASCII ranges, and `[\s\S]` preserves
multiline wildcard matching.

Rainbow queries follow Helix's query model where the grammar exposes structural delimiters.
Rainbow queries are omitted where the grammar has no structural bracket nodes.
See each language's README for pinned sources and adaptations.
