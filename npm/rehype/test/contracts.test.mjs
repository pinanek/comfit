import rehypeComfit from "@comfit/rehype";
import assert from "node:assert/strict";
import test from "node:test";
import { unified } from "unified";

function block(source, { language = "rust", meta, properties = {} } = {}) {
  const code = {
    type: "element",
    tagName: "code",
    properties: { className: [`language-${language}`], ...properties },
    data: { meta },
    children: [{ type: "text", value: source }],
  };
  const pre = { type: "element", tagName: "pre", properties: {}, children: [code] };
  return { tree: { type: "root", children: [pre] }, pre, code };
}

function transform(tree, options) {
  return unified().use(rehypeComfit, options).runSync(tree);
}

function text(node) {
  return node.type === "text" ? node.value : (node.children ?? []).map(text).join("");
}

for (
  const options of [
    {},
    { theme: "catppina-dark" },
    { style: "variables" },
    { themes: { light: "catppina-light", dark: "catppina-dark" }, defaultTheme: "dark" },
  ]
) {
  test(`preserves source and attributes: ${JSON.stringify(options)}`, () => {
    const source = "// <script>&\"'\0\u0001\u0002\r\nlet café = \"😀\";\r\n\n";
    const { tree, pre, code } = block(source, {
      properties: { id: "code-id", className: ["language-rust", "existing-code"] },
    });
    pre.properties = { id: "pre-id", className: ["existing-pre"], style: "padding:1rem" };
    transform(tree, { ...options, preClass: "added existing-pre", highlightedLines: [2], classPrefix: "syntax-" });
    assert.equal(text(code), source);
    assert.equal(pre.properties.id, "pre-id");
    assert.equal(code.properties.id, "code-id");
    assert.deepEqual(pre.properties.className, ["existing-pre", "added"]);
    assert(code.properties.className.includes("existing-code"));
    assert(pre.properties.style.startsWith("padding:1rem"));
    const lines = code.children.filter((child) => child.type === "element");
    assert.equal(lines.length, source.split("\n").length);
    assert(lines[1].properties.className.includes("syntax-line-highlight"));
  });
}

test("skips unknown languages by default and reports them when requested", () => {
  const { tree } = block("fn main() {}", { language: "not-loaded" });
  const before = structuredClone(tree);
  transform(tree);
  assert.deepEqual(tree, before);
  assert.throws(() => transform(tree, { unknownLanguage: "error" }), (error) => {
    assert.equal(error.source, "rehype-comfit");
    assert.equal(error.ruleId, "unknown-language");
    return true;
  });
});

test("supports custom aliases, default language, and custom wrapper tags", () => {
  const { tree, pre, code } = block("fn main() {}", { language: "custom" });
  transform(tree, { aliases: { custom: "rs" }, preTag: "div", lineTag: "div" });
  assert.equal(pre.tagName, "div");
  assert.equal(code.children[0].tagName, "div");
  assert.equal(code.children[0].children[0].properties.className[0], "keyword");
  const unlabelled = block("fn main() {}");
  delete unlabelled.code.properties.className;
  transform(unlabelled.tree, { defaultLanguage: "rust" });
  assert.equal(unlabelled.code.children[0].type, "element");
});

test("does not rewrite inline code, child markup, or unlabelled blocks", () => {
  const { tree, code } = block("fn main() {}");
  delete code.properties.className;
  tree.children.push(structuredClone(code));
  const markup = block("fn main() {}");
  markup.code.children = [{ type: "element", tagName: "b", properties: {}, children: [{ type: "text", value: "fn" }] }];
  tree.children.push(markup.pre);
  const sibling = block("fn main() {}");
  sibling.pre.children.push({ type: "text", value: "extra source" });
  tree.children.push(sibling.pre);
  const before = structuredClone(tree);
  transform(tree);
  assert.deepEqual(tree, before);
});

test("line metadata merges ranges and ignores quoted braces without expanding enormous ranges", () => {
  const source = "fn main() {\n let x = 1;\n let y = 2;\n}\n";
  const { tree, code } = block(source, { meta: "title=\"{1}\" {2-3,3} {5-9007199254740991}" });
  transform(tree, { highlightedLines: [4] });
  assert.deepEqual(
    code.children.filter((child) => child.type === "element").map((line) =>
      line.properties.className.includes("line-highlight")
    ),
    [false, true, true, true, true],
  );
  const disabled = block(source, { meta: "{2-3}" });
  transform(disabled.tree, { metaHighlight: false });
  assert(
    disabled.code.children.filter((child) => child.type === "element").every((line) =>
      !line.properties.className.includes("line-highlight")
    ),
  );
  for (const meta of ["{0}", "{3-2}", "{1,broken}", "{9007199254740992}"]) {
    assert.throws(() => transform(block(source, { meta }).tree), /Invalid highlighted-line range/);
  }
});

test("invalid option combinations fail before transforming source", () => {
  for (
    const options of [
      { theme: "catppina-dark", themes: { dark: "catppina-dark" } },
      { themes: {} },
      { themes: { dark: "catppina-dark" }, defaultTheme: "missing" },
      { defaultTheme: "dark" },
      { themes: { dark: "catppina-dark" }, style: "class" },
      { style: "inline" },
      { style: "invalid" },
      { unknownLanguage: "invalid" },
      { rainbowColors: -1 },
      { highlightedLines: [0] },
    ]
  ) {
    assert.throws(() => rehypeComfit(options), TypeError, JSON.stringify(options));
  }
});
