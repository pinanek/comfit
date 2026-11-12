import * as comfit from "@comfit/node";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const fixtures = JSON.parse(
  readFileSync(new URL("../../../crates/comfit/tests/fixtures/highlighting.json", import.meta.url)),
);

for (const fixture of fixtures) {
  test(fixture.name, () => {
    const config = new comfit.HighlightConfig();
    config.highlightedLines = fixture.highlightedLines;
    config.rainbow = fixture.rainbow;
    const lines = comfit.highlight(fixture.source, fixture.language, config);
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

    if (fixture.source.includes("\r")) {
      const html = comfit.renderHtmlClasses(fixture.source, fixture.language);
      assert(html.includes("&#13;"));
      assert(!html.includes("\r"));
    }
    const document = comfit.createDocument({ language: fixture.language, source: "" });
    try {
      document.update(fixture.source);
      assert.deepEqual(document.highlight(config).lines, lines);
    } finally {
      document.dispose();
    }
  });
}

test("documents invalidate caches and retain independent snapshots", () => {
  const source = "fn café() { let x = [1, (2)]; }\n";
  const document = comfit.createDocument({ language: "rs", source });
  const other = comfit.createDocument({ language: "rust", source: "fn other() {}" });
  const snapshot = document.highlight();
  const expected = comfit.highlight(source, "rust");

  try {
    for (const next of [source, "", "\n", "fn longer() { println!(\"😀\"); }\n", source]) {
      document.update(next);
      assert.equal(document.source, next);
      for (const rainbow of [false, true]) {
        for (const highlightedLines of [[1], [2], []]) {
          const config = new comfit.HighlightConfig();
          config.rainbow = rainbow;
          config.highlightedLines = highlightedLines;
          assert.deepEqual(document.highlight(config).lines, comfit.highlight(next, "rust", config));
        }
      }
    }
    assert.equal(other.source, "fn other() {}");
  } finally {
    document.dispose();
    document.dispose();
    other.dispose();
  }

  assert.deepEqual(snapshot.lines, expected);
  assert.equal(snapshot.renderHtmlClasses(), comfit.renderHtmlClasses(source, "rust"));
  assert.equal(snapshot.renderAnsi("catppina-dark"), comfit.renderAnsi(source, "rust", "catppina-dark"));
  assert.throws(() => document.source, /disposed/i);
  assert.throws(() => document.update(source), /disposed/i);
  assert.throws(() => document.highlight(), /disposed/i);
});

test("HTML modes and named themes render snapshots consistently", () => {
  const source = "fn main() { let x = [1]; }\n";
  const document = comfit.createDocument({ language: "rust", source });
  const snapshot = document.highlight();
  document.dispose();

  const config = new comfit.RenderHtmlConfig();
  config.highlightedLines = [1];
  config.classPrefix = "syntax-";
  config.preTag = "div";
  config.lineTag = "div";
  for (const style of [comfit.RenderHtmlStyle.Class, comfit.RenderHtmlStyle.Inline, comfit.RenderHtmlStyle.Variables]) {
    config.style = style;
    const html = comfit.renderHtml(source, "rust", "catppina-dark", config);
    assert.equal(snapshot.renderHtml("catppina-dark", config), html);
    assert(html.startsWith("<div"));
    assert(html.includes("syntax-line-highlight"));
    if (style === comfit.RenderHtmlStyle.Class) assert(html.includes("syntax-keyword-function"));
    if (style === comfit.RenderHtmlStyle.Inline) assert(html.includes("color:#"));
    if (style === comfit.RenderHtmlStyle.Variables) assert(html.includes("var(--comfit-keyword-fg,"));
  }
  const themes = [{ name: "day", theme: "catppina-light" }, { name: "night", theme: "catppina-dark" }];
  const html = comfit.renderHtmlThemes(source, "rust", themes, "night");
  assert.equal(snapshot.renderHtmlThemes(themes, "night"), html);
  assert(html.includes("--comfit-day"));
  assert(!html.includes("--comfit-night"));
  const dayHtml = comfit.renderHtmlThemes(source, "rust", themes, "day");
  assert(dayHtml.includes("--comfit-night"));
  assert.notEqual(html, dayHtml);
  assert.throws(() => comfit.renderHtmlThemes(source, "rust", themes, "missing"));
});

test("invalid public arguments report InvalidArg", () => {
  const invalid = (error) => error.code === "InvalidArg";
  assert.throws(() => comfit.highlight("", "not-a-language"), invalid);
  assert.throws(() => comfit.renderHtml("", "rust", "not-a-theme"), invalid);
  for (const value of [-1, 1.5, NaN, Infinity, Number.MAX_SAFE_INTEGER + 1]) {
    const config = new comfit.HighlightConfig();
    config.highlightedLines = [value];
    assert.throws(() => comfit.highlight("fn main() {}", "rust", config), invalid);
  }
  for (const value of [-1, 1.5, NaN, Infinity, Number.MAX_SAFE_INTEGER + 1]) {
    const config = new comfit.RenderHtmlConfig();
    config.rainbowColors = value;
    assert.throws(() => comfit.renderHtmlClasses("", "rust", config), invalid);
  }
});

test("HTML escapes source and attributes, ANSI cannot inject source controls", () => {
  const source = "// <script>&\"'\u001b[31m\0\t\n";
  const config = new comfit.RenderHtmlConfig();
  config.preClass = "\"<&'";
  const html = comfit.renderHtmlClasses(source, "rust", config);
  assert(html.includes("class=\"&quot;&lt;&amp;&#39;\""));
  assert(!html.includes("<script>"));
  assert(html.includes("&lt;script&gt;&amp;&quot;&#39;"));
  const ansi = comfit.renderAnsi(source, "rust", "catppina-dark");
  assert(ansi.includes("\\u{1b}[31m"));
  assert(ansi.includes("\\u{0}"));
  assert(!ansi.includes("\0"));
  assert(ansi.includes("\t"));
  assert(ansi.endsWith("\n"));
});
