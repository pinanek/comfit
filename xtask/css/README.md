# CSS generation

Run from the repository root:

```sh
cargo css build
cargo css build --class-prefix hl-
```

Generate stylesheets from `crates/comfit_themes/data` in `npm/css/dist`. Match `--class-prefix` to the HTML renderer's `classPrefix` option.

See [@comfit/css](../../npm/css/README.md) for imports and renderer options.
