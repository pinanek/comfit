import { defineConfig } from "oxlint";
import baseConfig from "../oxlint.config.ts";

export default defineConfig({
  extends: [baseConfig],
  plugins: ["react", "jsx-a11y"],
  categories: {
    correctness: "error",
  },
  env: {
    browser: true,
  },
  rules: {
    // Scrollable code and table regions must remain keyboard-focusable.
    "jsx-a11y/no-noninteractive-tabindex": ["error", { tags: ["pre", "section"] }],
  },
  ignorePatterns: ["node_modules/**", "dist/**", "content/api-references/**"],
});
