/** Highlight fenced code blocks in markdown-it with Comfit. */

import {
  containsLanguage,
  renderHtml,
  renderHtmlClasses,
  RenderHtmlConfig,
  RenderHtmlStyle,
  renderHtmlThemes,
} from "@comfit/node";
import type MarkdownIt from "markdown-it";

/** Language selection, themes, and rendering options for fenced code blocks. */
export interface Options {
  /** Skip unknown languages or throw. Defaults to `"skip"`. */
  unknownLanguage?: "skip" | "error";
  /** Language for fences without a language name. Otherwise leave them unchanged. */
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
  /** Rainbow class wrapping count. Zero preserves depth without a theme palette. */
  rainbowColors?: number;
  /** Prefix for generated token, rainbow, and line classes. Defaults to an empty string. */
  classPrefix?: string;
  /** Class on the outer pre element, escaped as attribute text. */
  preClass?: string;
  /** One-based source lines to emphasize in every fence. Defaults to an empty list. */
  highlightedLines?: readonly number[];
}

/**
 * Highlight fenced code blocks while preserving markdown-it's language classes and token attributes.
 *
 * Inline code, indented code, and unlabelled fences without a default language are unchanged.
 * Unknown languages use the previous fence renderer unless configured as errors.
 * Calls on the same Node worker share a synchronous highlighting engine.
 */
export default function markdownItComfit(md: InstanceType<typeof MarkdownIt>, options: Options = {}): void {
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
  config.highlightedLines = [...options.highlightedLines ?? []];
  config.style = {
    class: RenderHtmlStyle.Class,
    inline: RenderHtmlStyle.Inline,
    variables: RenderHtmlStyle.Variables,
  }[style];
  if (options.preClass !== undefined) config.preClass = options.preClass;

  const fallback = md.renderer.rules.fence!;
  md.renderer.rules.fence = (tokens, index, renderOptions, env, renderer) => {
    const token = tokens[index]!;
    const info = md.utils.unescapeAll(token.info).trim();
    const requested = info.split(/\s+/, 1)[0] || options.defaultLanguage;
    if (requested === undefined) return fallback(tokens, index, renderOptions, env, renderer);
    const language = Object.hasOwn(options.aliases ?? {}, requested) ? options.aliases![requested]! : requested;

    if (!containsLanguage(language)) {
      if (options.unknownLanguage === "error") throw new Error(`Unknown language: ${requested}`);
      return fallback(tokens, index, renderOptions, env, renderer);
    }

    const html = named.length > 0
      ? renderHtmlThemes(token.content, language, named, defaultTheme!, config)
      : options.theme !== undefined
      ? renderHtml(token.content, language, options.theme, config)
      : renderHtmlClasses(token.content, language, config);
    const attributes = (token.attrs ?? []).map(([name, value]): [string, string] => [name, String(value)]);
    const languageClass = `${renderOptions.langPrefix ?? "language-"}${requested}`;
    const classes = attributes.find(([name]) => name === "class");
    if (classes) classes[1] = `${classes[1]} ${languageClass}`;
    else attributes.push(["class", languageClass]);
    const codeAttributes = attributes.map(([name, value]) =>
      ` ${md.utils.escapeHtml(name)}="${md.utils.escapeHtml(value)}"`
    ).join("");

    return `${html.replace("<code>", `<code${codeAttributes}>`)}\n`;
  };
}
