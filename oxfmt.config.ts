import { defineConfig } from "oxfmt";

export default defineConfig({
  printWidth: 120,
  tabWidth: 2,
  useTabs: false,
  semi: true,
  singleQuote: false,
  trailingComma: "all",
  proseWrap: "preserve",
  sortPackageJson: false,
  associations: [{ files: ["docs/content/**/*.md"], language: "mdx" }],
  ignorePatterns: [
    "**/node_modules/**",
    "**/dist/**",
    "**/target/**",
    ".grammars/**",
    "npm/wasm/js/generated/**",
    "npm/wasm-grammars/generated/**",
    "crates/comfit_grammars/data/**",
    "crates/comfit_themes/data/**",
    "docs/content/api-references/**",
    "**/pnpm-lock.yaml",
    "uv.lock",
    "Cargo.lock",
  ],
});
