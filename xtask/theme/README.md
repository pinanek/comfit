# Theme import

Run from the repository root:

```sh
cargo theme update
cargo theme update --theme catppuccin-frappe --debug
```

Import themes from `registry/themes.toml` into `crates/comfit_themes/data`. Use `--theme` to select one theme and `--debug` to list unmapped selectors and occurrence counts.

Preview an imported theme:

```sh
cargo run -p comfit-cli -- ansi xtask/theme/src/main.rs \
  --theme-file crates/comfit_themes/data/catppuccin-frappe.toml
```

When adding a theme, declare its feature in `crates/comfit_themes/Cargo.toml` and include it in `all-themes`.
