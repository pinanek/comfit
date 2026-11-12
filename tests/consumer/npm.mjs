import * as native from "@comfit/node";
import rehypeComfit from "@comfit/rehype";
import { createHighlighter, initialize } from "@comfit/wasm";
import rust from "@comfit/wasm-grammars/rust";
import assert from "node:assert/strict";
import { readdir, readFile } from "node:fs/promises";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

const source = "fn café() { let x = [1]; }\n";
const expected = native.highlight(source, "rs");
assert.equal(typeof rehypeComfit(), "function");
await initialize({
  engine: await readFile(new URL(import.meta.resolve("@comfit/wasm/comfit_bg.wasm"))),
  treeSitter: await readFile(new URL(import.meta.resolve("@comfit/wasm/web-tree-sitter.wasm"))),
});
const highlighter = await createHighlighter({ languages: [{ ...rust, wasm: await readFile(new URL(rust.wasm)) }] });
try {
  assert.deepEqual(highlighter.highlight(source, "rs"), expected);
  assert.equal(highlighter.renderHtml(source, "rs"), native.renderHtmlClasses(source, "rs"));
} finally {
  highlighter.dispose();
}

const cssDirectory = dirname(fileURLToPath(import.meta.resolve("@comfit/css/catppina-dark.css")));
const stylesheets = (await readdir(cssDirectory)).filter((name) => name.endsWith(".css"));
assert.deepEqual(stylesheets.map((name) => name.slice(0, -4)).sort(), native.themes().sort());
for (const name of stylesheets) {
  const css = await readFile(new URL(import.meta.resolve(`@comfit/css/${name}`)), "utf8");
  assert(css.includes(`pre.comfit-${name.slice(0, -4)}`));
}

const grammarDirectory = dirname(fileURLToPath(import.meta.resolve("@comfit/wasm-grammars/rust")));
const descriptors = (await readdir(grammarDirectory)).filter((name) => name.endsWith(".js") && name !== "types.js");
const ids = [];
for (const name of descriptors) {
  const specifier = `@comfit/wasm-grammars/${name.slice(0, -3)}`;
  const { default: language } = await import(specifier);
  ids.push(language.id);
  const asset = new URL(language.wasm);
  assert.equal(import.meta.resolve(`@comfit/wasm-grammars/${asset.pathname.split("/").at(-1)}`), asset.href);
  const bytes = await readFile(asset);
  assert(WebAssembly.validate(bytes), `Invalid grammar WASM: ${name}`);
  await readFile(new URL(import.meta.resolve(specifier).replace(/\.js$/, ".d.ts")));
}
assert.deepEqual(ids.sort(), native.languages().sort());
console.log(`Installed npm exports verified: ${ids.length} grammars and ${stylesheets.length} themes.`);
