import { createHighlighter, initialize } from "@comfit/wasm";
import javascript from "@comfit/wasm-grammars/javascript";
import markdown from "@comfit/wasm-grammars/markdown";
import markdownInline from "@comfit/wasm-grammars/markdown-inline";
import rust from "@comfit/wasm-grammars/rust";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { after, test } from "node:test";

const fixtures = JSON.parse(
  await readFile(new URL("../../../crates/comfit/tests/fixtures/highlighting.json", import.meta.url)),
);
const languages = await Promise.all([rust, javascript, markdown, markdownInline].map(async (language) => ({
  ...language,
  wasm: await readFile(new URL(language.wasm)),
})));

await test("initialization rejects missing/invalid sources and supports concurrent retry", async () => {
  await assert.rejects(createHighlighter({ languages: [] }), /initialize/i);
  await assert.rejects(initialize({}), /requires/i);
  await assert.rejects(initialize({ engine: new Uint8Array([0]), treeSitter: new Uint8Array([0]) }));
  const options = {
    engine: await readFile(new URL(import.meta.resolve("@comfit/wasm/comfit_bg.wasm"))),
    treeSitter: await readFile(new URL(import.meta.resolve("@comfit/wasm/web-tree-sitter.wasm"))),
  };
  await Promise.all([initialize(options), initialize(options)]);
  await initialize({ engine: new Uint8Array([0]), treeSitter: new Uint8Array([0]) });
});

const highlighter = await createHighlighter({ languages });
after(() => highlighter.dispose());

for (const fixture of fixtures) {
  test(fixture.name, () => {
    const options = { highlightedLines: fixture.highlightedLines, rainbow: fixture.rainbow };
    const lines = highlighter.highlight(fixture.source, fixture.language, options);
    assert.deepEqual(
      lines.map((line) => line.tokens.map(({ text, highlight, rainbow }) => [text, highlight, rainbow])),
      fixture.tokens,
    );
    assert.equal(lines.map((line) => line.tokens.map((token) => token.text).join("")).join("\n"), fixture.source);
    assert.equal(lines.length, fixture.source.split("\n").length);
    for (const [index, line] of lines.entries()) {
      assert.equal(line.number, index + 1);
      assert.equal(line.highlighted, fixture.highlightedLines.includes(index + 1));
    }
    const document = highlighter.createDocument({ language: fixture.language, source: "" });
    try {
      document.update(fixture.source);
      const result = document.highlight(options);
      try {
        assert.equal(result.lineCount, lines.length);
        assert.deepEqual(result.lines, lines);
      } finally {
        result.dispose();
      }
    } finally {
      document.dispose();
    }
  });
}

test("documents invalidate caches, own snapshots, and outlive their registry", async () => {
  const registry = await createHighlighter({ languages });
  const source = "fn café() { let x = [1, (2)]; }\n";
  const document = registry.createDocument({ language: "rs", source });
  const other = registry.createDocument({ language: "rust", source: "fn other() {}" });
  const snapshot = document.highlight();
  const expected = registry.highlight(source, "rust");
  const html = registry.renderHtml(source, "rust");
  registry.dispose();
  registry.dispose();

  try {
    for (const next of [source, "", "\n", "fn longer() { println!(\"😀\"); }\n", source]) {
      document.update(next);
      assert.equal(document.source, next);
      for (const rainbow of [false, true]) {
        for (const highlightedLines of [[1], [2], []]) {
          const options = { rainbow, highlightedLines };
          const result = document.highlight(options);
          try {
            assert.deepEqual(result.lines, highlighter.highlight(next, "rust", options));
            assert.equal(result.renderHtml(), highlighter.renderHtml(next, "rust", options));
          } finally {
            result.dispose();
          }
        }
      }
    }
    assert.equal(other.source, "fn other() {}");
  } finally {
    document.dispose();
    document.dispose();
    other.dispose();
  }

  try {
    assert.deepEqual(snapshot.lines, expected);
    assert.equal(snapshot.renderHtml(), html);
  } finally {
    snapshot.dispose();
    snapshot.dispose();
  }
  assert.deepEqual(snapshot.lines, expected);
  assert.throws(() => snapshot.renderHtml(), /disposed/i);
  assert.throws(() => document.source, /disposed/i);
  assert.throws(() => document.update(source), /disposed/i);
  assert.throws(() => document.highlight(), /disposed/i);
  assert.throws(() => registry.highlight(source, "rust"), /disposed/i);
  assert.throws(() => registry.createDocument({ language: "rust", source }), /disposed/i);
});

test("unexported result lines cannot be read after disposal", () => {
  const document = highlighter.createDocument({ language: "rust", source: "" });
  const result = document.highlight();
  document.dispose();
  result.dispose();
  assert.throws(() => result.lines, /disposed/i);
});

test("invalid grammar registration does not poison subsequent highlighters", async () => {
  await assert.rejects(createHighlighter({ languages: [{ ...languages[0], highlights: "(not_a_node) @variable" }] }));
  await assert.rejects(createHighlighter({ languages: [languages[0], languages[0]] }));
  const registry = await createHighlighter({ languages: [languages[0]] });
  try {
    assert.deepEqual(registry.highlight("fn main() {}", "rs"), highlighter.highlight("fn main() {}", "rust"));
  } finally {
    registry.dispose();
  }
});

test("HTML modes, inherited styles, and named themes render snapshots consistently", () => {
  const source = "fn main() { let x = [1]; }\n";
  const document = highlighter.createDocument({ language: "rust", source });
  const snapshot = document.highlight();
  document.dispose();
  const theme = {
    default: { fg: "#111111", bg: "#ffffff" },
    styles: { keyword: { fg: "#ff0000" } },
    rainbow: ["#00ff00"],
    lineHighlight: { bg: "#eeeeee" },
  };
  try {
    for (
      const rendering of [
        { style: "class" },
        { style: "variables" },
        { style: "inline", theme },
        { style: "inline", themes: { day: theme, night: { default: { fg: "#eeeeee" } } }, defaultTheme: "night" },
      ]
    ) {
      const options = { ...rendering, classPrefix: "syntax-", highlightedLines: [1], preTag: "div", lineTag: "div" };
      const html = highlighter.renderHtml(source, "rust", options);
      assert.equal(snapshot.renderHtml(options), html);
      assert(html.startsWith("<div"));
      assert(html.includes("syntax-line-highlight"));
      if (rendering.style === "class") assert(html.includes("syntax-keyword-function"));
      if (rendering.style === "variables") assert(html.includes("var(--comfit-keyword-fg,"));
      if (rendering.theme) {
        assert(html.includes("color:#ff0000;"));
        assert(html.includes("color:#00ff00;"));
        assert(html.includes("background-color:#eeeeee;"));
      }
      if (rendering.themes) {
        assert(html.includes("--comfit-day"));
        assert(!html.includes("--comfit-night"));
        assert(html.includes("color:#eeeeee;"));
      }
    }
    assert.throws(() => snapshot.renderHtml({ style: "inline" }));
    assert.throws(() => snapshot.renderHtml({ style: "inline", themes: { day: theme }, defaultTheme: "missing" }));
  } finally {
    snapshot.dispose();
  }
});

test("invalid language and numeric options throw", () => {
  assert.throws(() => highlighter.highlight("", "not-a-language"));
  for (const value of [-1, 1.5, NaN, Infinity, Number.MAX_SAFE_INTEGER + 1]) {
    assert.throws(() => highlighter.highlight("", "rust", { highlightedLines: [value] }));
    assert.throws(() => highlighter.renderHtml("", "rust", { rainbowColors: value }));
  }
});
