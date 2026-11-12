import { expect, test } from "@playwright/test";
import { readFileSync } from "node:fs";

const fixtures = JSON.parse(
  readFileSync(new URL("../../../crates/comfit/tests/fixtures/highlighting.json", import.meta.url)),
);

for (const fixture of fixtures) {
  test(fixture.name, async ({ page }) => {
    await page.goto("/tests/browser/index.html");
    const result = await page.evaluate(async (fixture) => {
      const { initialize, createHighlighter } = await import("/npm/wasm/dist/index.js");
      await initialize({ engine: "/npm/wasm/dist/comfit_bg.wasm", treeSitter: "/npm/wasm/dist/web-tree-sitter.wasm" });
      const languages = await Promise.all(
        ["rust", "javascript", "markdown", "markdown-inline"].map(async (id) =>
          (await import(`/npm/wasm-grammars/dist/${id}.js`)).default
        ),
      );
      const highlighter = await createHighlighter({ languages });
      const document = highlighter.createDocument({ language: fixture.language, source: "" });
      let snapshot;
      try {
        document.update(fixture.source);
        snapshot = document.highlight({ rainbow: fixture.rainbow, highlightedLines: fixture.highlightedLines });
      } finally {
        document.dispose();
        highlighter.dispose();
      }
      try {
        return { lines: snapshot.lines, lineCount: snapshot.lineCount, html: snapshot.renderHtml() };
      } finally {
        snapshot.dispose();
      }
    }, fixture);
    expect(result.lines.map((line) => line.tokens.map(({ text, highlight, rainbow }) => [text, highlight, rainbow])))
      .toEqual(fixture.tokens);
    expect(result.lineCount).toBe(fixture.source.split("\n").length);
    expect(result.lines.map((line) => line.number)).toEqual(result.lines.map((_, index) => index + 1));
    expect(result.lines.map((line) => line.highlighted)).toEqual(
      result.lines.map((_, index) => fixture.highlightedLines.includes(index + 1)),
    );
    await page.setContent(result.html);
    expect(await page.locator("pre > code").textContent()).toBe(fixture.source);
  });
}

test("failed initialization can be retried and concurrent calls share downloads", async ({ page }) => {
  let requests = 0;
  await page.route("**/comfit_bg.wasm", async (route) => {
    requests++;
    if (requests === 1) await route.fulfill({ status: 503, body: "Unavailable" });
    else await route.continue();
  });
  await page.goto("/tests/browser/index.html");
  const result = await page.evaluate(async () => {
    const { initialize, createHighlighter } = await import("/npm/wasm/dist/index.js");
    const options = { engine: "/npm/wasm/dist/comfit_bg.wasm", treeSitter: "/npm/wasm/dist/web-tree-sitter.wasm" };
    let failed = false;
    try {
      await initialize(options);
    } catch (error) {
      failed = error.message.includes("503");
    }
    await Promise.all([initialize(options), initialize(options)]);
    const highlighter = await createHighlighter({ languages: [] });
    highlighter.dispose();
    return failed;
  });
  expect(result).toBe(true);
  expect(requests).toBe(2);
});

test("failed grammar fetch does not poison later highlighters", async ({ page }) => {
  let requests = 0;
  await page.route("**/tree-sitter-rust.wasm", async (route) => {
    requests++;
    if (requests === 1) await route.fulfill({ status: 503, body: "Unavailable" });
    else await route.continue();
  });
  await page.goto("/tests/browser/index.html");
  const result = await page.evaluate(async () => {
    const { initialize, createHighlighter } = await import("/npm/wasm/dist/index.js");
    const { default: rust } = await import("/npm/wasm-grammars/dist/rust.js");
    await initialize({ engine: "/npm/wasm/dist/comfit_bg.wasm", treeSitter: "/npm/wasm/dist/web-tree-sitter.wasm" });
    let failed = false;
    try {
      await createHighlighter({ languages: [rust] });
    } catch (error) {
      failed = error.message.includes("503");
    }
    const highlighter = await createHighlighter({ languages: [rust] });
    try {
      return { failed, lines: highlighter.highlight("fn main() {}", "rs") };
    } finally {
      highlighter.dispose();
    }
  });
  expect(result.failed).toBe(true);
  expect(result.lines[0].tokens[0].highlight).toBe("keyword.function");
  expect(requests).toBe(2);
});
