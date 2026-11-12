import type { UserConfig } from "tsdown";

const config: UserConfig = {
  entry: { index: "./src/index.ts" },
  platform: "node",
  target: "node22",
  format: "esm",
  fixedExtension: false,
  dts: true,
  exports: true,
};

export default config;
