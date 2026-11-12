/** Highlight code blocks in HAST with Comfit. */

import {
  containsLanguage,
  highlight,
  HighlightConfig,
  renderHtml,
  renderHtmlClasses,
  RenderHtmlConfig,
  RenderHtmlStyle,
  renderHtmlThemes,
} from "@comfit/node";
import type { Element, Properties, Root } from "hast";
import { fromHtml } from "hast-util-from-html";
import type { Transformer } from "unified";
import { SKIP, visit } from "unist-util-visit";

declare module "hast" {
  interface ElementData {
    /** Fence metadata preserved by remark-rehype. */
    meta?: string | null | undefined;
  }
}

/** Rendering, language selection, and line emphasis for code blocks. */
export interface Options {
  /** Skip unknown languages or fail the file. Defaults to `"skip"`. */
  unknownLanguage?: "skip" | "error";
  /** Language for blocks without a language class. Otherwise leave them unchanged. */
  defaultLanguage?: string;
  /** Additional aliases, resolved once before checking built-in IDs and aliases. */
  aliases?: Readonly<Record<string, string>>;
  /** Bundled theme ID. Cannot be combined with `themes`. */
  theme?: string;
  /** Named bundled themes for inline output. The first name is the default unless selected. */
  themes?: Readonly<Record<string, string>>;
  /** Default name in `themes`. Requires `themes`. */
  defaultTheme?: string;
  /** Defaults to `"inline"` with themes, otherwise `"class"`. Inline output requires a theme. */
  style?: "class" | "inline" | "variables";
  /** Collect rainbow bracket depths. Defaults to `true`. */
  rainbow?: boolean;
  /** Rainbow class wrapping count. Zero preserves depth without a theme palette. Defaults to zero. */
  rainbowColors?: number;
  /** Prefix for generated token, rainbow, and line classes. Defaults to an empty string. */
  classPrefix?: string;
  /** Outer tag name. Defaults to "pre". */
  preTag?: string;
  /** Line tag name. Defaults to "span". */
  lineTag?: string;
  /** Classes appended to the existing outer element's classes. */
  preClass?: string;
  /** One-based source lines to emphasize in every block. Defaults to an empty list. */
  highlightedLines?: readonly number[];
  /** Read line ranges such as `{2,4-6}` from code.data.meta. Defaults to `true`. */
  metaHighlight?: boolean;
}

/**
 * Highlight text-only pre/code blocks, preserving their attributes and source text.
 *
 * Inline code, blocks with child markup, and unlabelled blocks without a default
 * language remain unchanged. Unknown languages are skipped unless configured as
 * errors. Rendering failures and invalid line metadata are reported through the VFile.
 * Calls on the same Node worker share a synchronous highlighting engine.
 */
export default function rehypeComfit(options: Options = {}): Transformer<Root> {
  const named = Object.entries(options.themes ?? {}).map(([name, theme]) => ({ name, theme }));
  const defaultTheme = options.defaultTheme ?? named[0]?.name;
  const style = options.style ?? (options.theme !== undefined || options.themes !== undefined ? "inline" : "class");

  if (options.theme !== undefined && options.themes !== undefined) {
    throw new TypeError("theme and themes cannot be combined");
  }
  if (options.themes !== undefined && (named.length === 0 || !named.some(({ name }) => name === defaultTheme))) {
    throw new TypeError("themes must contain the default theme");
  }
  if (options.defaultTheme !== undefined && options.themes === undefined) {
    throw new TypeError("defaultTheme requires themes");
  }
  if (options.themes !== undefined && style !== "inline") {
    throw new TypeError("named themes require inline style");
  }
  if (style === "inline" && options.theme === undefined && named.length === 0) {
    throw new TypeError("inline style requires theme or themes");
  }
  if (!["class", "inline", "variables"].includes(style)) {
    throw new TypeError("style must be class, inline, or variables");
  }
  if (options.unknownLanguage !== undefined && !["skip", "error"].includes(options.unknownLanguage)) {
    throw new TypeError("unknownLanguage must be skip or error");
  }
  if (!Number.isSafeInteger(options.rainbowColors ?? 0) || (options.rainbowColors ?? 0) < 0) {
    throw new TypeError("rainbowColors must be a nonnegative safe integer");
  }
  for (const line of options.highlightedLines ?? []) {
    if (!Number.isSafeInteger(line) || line < 1) {
      throw new TypeError("highlightedLines must contain positive safe integers");
    }
  }

  const config = new RenderHtmlConfig();
  config.rainbow = options.rainbow ?? true;
  config.rainbowColors = options.rainbowColors ?? 0;
  config.classPrefix = options.classPrefix ?? "";
  config.style = {
    class: RenderHtmlStyle.Class,
    inline: RenderHtmlStyle.Inline,
    variables: RenderHtmlStyle.Variables,
  }[style];
  if (options.preClass !== undefined) config.preClass = options.preClass;

  return (tree, file) => {
    visit(tree, "element", (pre) => {
      if (pre.tagName !== "pre") return;
      const code = pre.children.find((child): child is Element => child.type === "element" && child.tagName === "code");
      if (
        !code || !code.children.every((child) => child.type === "text")
        || pre.children.some((child) => child !== code && (child.type !== "text" || child.value.trim() !== ""))
      ) return SKIP;

      const languageClass = classes(code.properties.className).find((name) => /^(?:language|lang)-./.test(name));
      const requested = languageClass?.replace(/^(?:language|lang)-/, "") ?? options.defaultLanguage;
      if (requested === undefined) return SKIP;
      const language = Object.hasOwn(options.aliases ?? {}, requested) ? options.aliases![requested]! : requested;

      if (!containsLanguage(language)) {
        if (options.unknownLanguage === "error") {
          file.fail(`Unknown language: ${requested}`, code, "rehype-comfit:unknown-language");
        }
        return SKIP;
      }

      const source = code.children.map((child) => child.type === "text" ? child.value : "").join("");
      try {
        config.highlightedLines = selectedLines(source, code, options);
        let rendered: Element;
        if (style === "class" && options.theme === undefined) {
          rendered = renderClasses(source, language, config);
        } else {
          const html = named.length > 0
            ? renderHtmlThemes(source, language, named, defaultTheme!, config)
            : options.theme !== undefined
            ? renderHtml(source, language, options.theme, config)
            : renderHtmlClasses(source, language, config);
          rendered = parseRendered(html);
        }
        const renderedCode = rendered.children.find((child): child is Element =>
          child.type === "element" && child.tagName === "code"
        );
        if (!renderedCode) throw new Error("Comfit did not render a code element");
        // Apply tag names after parsing so HTML parser rules cannot rewrite the wrappers.
        pre.tagName = options.preTag ?? rendered.tagName;
        if (options.lineTag !== undefined) {
          for (const child of renderedCode.children) {
            if (child.type === "element") child.tagName = options.lineTag;
          }
        }
        mergeProperties(pre.properties, rendered.properties);
        mergeProperties(code.properties, renderedCode.properties);
        code.children = renderedCode.children;
      } catch (error) {
        file.fail(error instanceof Error ? error : new Error(String(error)), code, "rehype-comfit:highlight");
      }
      return SKIP;
    });
  };
}

function renderClasses(source: string, language: string, config: RenderHtmlConfig): Element {
  // Class-only output can be built directly without serializing and parsing HTML.
  const highlighting = new HighlightConfig();
  highlighting.rainbow = config.rainbow;
  highlighting.highlightedLines = config.highlightedLines;
  const lines = highlight(source, language, highlighting);
  const rendered: Element = { type: "element", tagName: "pre", properties: {}, children: [] };
  if (typeof config.preClass === "string") rendered.properties.className = classes(config.preClass);
  const output: Element = { type: "element", tagName: "code", properties: {}, children: [] };
  rendered.children.push(output);
  for (const [index, line] of lines.entries()) {
    if (index > 0) output.children.push({ type: "text", value: "\n" });
    const lineClasses = [`${config.classPrefix}line`];
    if (line.highlighted) lineClasses.push(`${config.classPrefix}line-highlight`);
    const lineNode: Element = {
      type: "element",
      tagName: "span",
      properties: { className: lineClasses },
      children: [],
    };
    output.children.push(lineNode);
    for (const token of line.tokens) {
      const names: string[] = [];
      if (token.highlight !== null) {
        let prefix = "";
        for (const part of token.highlight.split(".")) {
          if (prefix !== "") prefix += "-";
          prefix += part;
          names.push(`${config.classPrefix}${prefix}`);
        }
      }
      if (token.rainbow !== null) {
        const color = config.rainbowColors === 0 ? token.rainbow : token.rainbow % config.rainbowColors;
        names.push(`${config.classPrefix}rainbow`, `${config.classPrefix}rainbow-${color}`);
      }
      lineNode.children.push({
        type: "element",
        tagName: "span",
        properties: names.length > 0 ? { className: names } : {},
        children: [{ type: "text", value: token.text }],
      });
    }
  }
  return rendered;
}

function classes(value: Properties[string]): string[] {
  return Array.isArray(value) ? value.map(String) : typeof value === "string" ? value.split(/\s+/).filter(Boolean) : [];
}

function mergeProperties(target: Properties, rendered: Properties): void {
  for (const [key, value] of Object.entries(rendered)) {
    if (key === "className") {
      target.className = [...new Set([...classes(target.className), ...classes(value)])];
    } else if (key === "style" && typeof target.style === "string" && target.style !== "") {
      target.style = `${target.style.replace(/;?\s*$/, ";")}${value}`;
    } else {
      target[key] = value;
    }
  }
}

function selectedLines(source: string, code: Element, options: Options): number[] {
  const selected = new Set(options.highlightedLines);
  const meta = options.metaHighlight !== false && typeof code.data?.meta === "string" ? code.data.meta : "";
  const ranges: [number, number][] = [];
  // Quoted metadata values can contain braces without selecting lines.
  for (const match of meta.matchAll(/"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|(?:^|\s)\{([^{}]*)\}(?=\s|$)/g)) {
    if (match[1] === undefined) continue;
    const value = match[1].trim();
    if (!/^[+-]?\d/.test(value)) continue;
    for (const part of value.split(",")) {
      const range = /^\s*(\d+)(?:\s*-\s*(\d+))?\s*$/.exec(part);
      if (!range) throw new Error(`Invalid highlighted-line range: {${value}}`);
      const start = Number(range[1]);
      const end = Number(range[2] ?? range[1]);
      if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end) || start < 1 || end < start) {
        throw new Error(`Invalid highlighted-line range: {${value}}`);
      }
      ranges.push([start, end]);
    }
  }
  if (ranges.length > 0) {
    ranges.sort(([left], [right]) => left - right);
    let cursor = 0;
    let number = 1;
    // Sweep actual lines rather than expanding ranges supplied by the document.
    for (let index = 0; index <= source.length && cursor < ranges.length; index++) {
      if (index === source.length || source[index] === "\n") {
        while (cursor < ranges.length && ranges[cursor]![1] < number) cursor++;
        if (cursor < ranges.length && ranges[cursor]![0] <= number) selected.add(number);
        number++;
      }
    }
  }
  return [...selected];
}

function parseRendered(html: string): Element {
  // HTML parsing normalizes carriage returns and replaces NUL. Preserve source text.
  // A distinct terminator keeps adjacent source controls outside the placeholder.
  let prefix = "\u0001";
  while (html.includes(`${prefix}\u0002`)) prefix += prefix;
  const marker = `${prefix}\u0002`;
  const tree = fromHtml(html.replaceAll("\r", "&#13;").replaceAll("\0", marker), { fragment: true });
  visit(tree, (node) => {
    delete node.position;
    if (node.type === "text") node.value = node.value.replaceAll(marker, "\0");
  });
  const pre = tree.children.find((child): child is Element => child.type === "element" && child.tagName === "pre");
  if (!pre) throw new Error("Comfit did not render a pre element");
  return pre;
}
