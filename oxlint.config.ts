import { defineConfig } from "oxlint";

export default defineConfig({
  plugins: ["typescript", "unicorn", "oxc"],
  categories: {
    correctness: "error",
  },
  rules: {
    "no-debugger": "error",
  },
  ignorePatterns: [
    "**/node_modules/**",
    "**/dist/**",
    "**/target/**",
    ".grammars/**",
    "npm/wasm/js/generated/**",
    "npm/wasm-grammars/generated/**",
  ],
});
