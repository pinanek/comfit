/** Load explicit WASM bytes and highlight JavaScript from Node.js. */

import { createHighlighter, initialize } from "@comfit/wasm";
import javascript from "@comfit/wasm-grammars/javascript";
import { readFile } from "node:fs/promises";

await initialize({
  engine: await readFile(
    new URL(import.meta.resolve("@comfit/wasm/comfit_bg.wasm")),
  ),
  treeSitter: await readFile(
    new URL(import.meta.resolve("@comfit/wasm/web-tree-sitter.wasm")),
  ),
});

const highlighter = await createHighlighter({
  languages: [{
    ...javascript,
    wasm: await readFile(
      new URL(import.meta.resolve("@comfit/wasm-grammars/tree-sitter-javascript.wasm")),
    ),
  }],
});

try {
  console.log(highlighter.renderHtml("const answer = 42;", "javascript"));
} finally {
  highlighter.dispose();
}
