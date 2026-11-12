import { createDocument, HighlightConfig, type Line, renderHtmlClasses } from "@comfit/node";
import rehypeComfit from "@comfit/rehype";
import { createHighlighter, type HighlightResult, initialize, type RenderHtmlOptions } from "@comfit/wasm";
import rust from "@comfit/wasm-grammars/rust";

const document = createDocument({ language: "rust", source: "fn main() {}" });
const config = new HighlightConfig();
config.highlightedLines = [1];
const lines: Line[] = document.highlight(config).lines;
renderHtmlClasses(lines[0].tokens[0].text, "rust");
document.dispose();
rehypeComfit({ themes: { dark: "catppina-dark" }, defaultTheme: "dark" });

await initialize({ engine: new Uint8Array(), treeSitter: new ArrayBuffer(0) });
const highlighter = await createHighlighter({ languages: [rust] });
const session = highlighter.createDocument({ language: "rust", source: "" });
const result: HighlightResult = session.highlight({ rainbow: true });
result.renderHtml({ style: "inline", theme: { default: { fg: "#fff" } } });
result.dispose();
session.dispose();
highlighter.dispose();

// @ts-expect-error Inline rendering requires a theme.
const missingTheme: RenderHtmlOptions = { style: "inline" };
// @ts-expect-error Theme values are not accepted in class mode.
const classTheme: RenderHtmlOptions = { style: "class", theme: {} };
// @ts-expect-error A single theme and named themes are mutually exclusive.
const mixedThemes: RenderHtmlOptions = { style: "inline", theme: {}, themes: { dark: {} } };
void [missingTheme, classTheme, mixedThemes];
