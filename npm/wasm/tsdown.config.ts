/** Bundle the browser entry point and copy its WASM runtimes. */

import type { UserConfig } from "tsdown";

/** Keep runtime WASM adjacent to the entry point so module-relative asset paths resolve. */
const config: UserConfig = {
  entry: { index: "./js/index.ts" },
  platform: "browser",
  fixedExtension: false,
  outDir: "dist",
  copy: [
    "./js/generated/*.wasm",
    "./node_modules/web-tree-sitter/web-tree-sitter.wasm",
  ],
};

export default config;
