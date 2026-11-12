/** Bundle generated grammar descriptors with their WASM assets. */

import { readdirSync } from "node:fs";
import type { UserConfig } from "tsdown";

const languages = readdirSync("./generated")
  .filter(file => file.endsWith(".ts"))
  .map(file => [file.slice(0, -3), `./generated/${file}`]);

/** Preserve per-language entry points without a root module that imports every grammar. */
const config: UserConfig = {
  entry: { types: "./src/types.ts", ...Object.fromEntries(languages) },
  platform: "browser",
  fixedExtension: false,
  outDir: "dist",
  copy: ["./generated/*.wasm"],
};

export default config;
