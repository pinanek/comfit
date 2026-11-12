import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./npm/wasm/test",
  testMatch: "**/*.spec.mjs",
  use: { baseURL: "http://127.0.0.1:4174", browserName: "chromium" },
  webServer: {
    command: "python3 -m http.server 4174 --bind 127.0.0.1",
    url: "http://127.0.0.1:4174/tests/browser/index.html",
  },
});
