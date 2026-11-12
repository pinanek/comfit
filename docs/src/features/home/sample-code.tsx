import { springConfig } from "#/lib/motion";
import { CodeBlock } from "#/mdx/code-block";
import type { Theme } from "@comfit/wasm";
import { animated, useReducedMotion, useSpring } from "@react-spring/web";
import { type CSSProperties, Fragment, useRef } from "react";
import { sampleLines, sampleSource } from "./sample";
import styles from "./sample-code.module.css";
import { TokenCallout } from "./token-callout";

export type Scene = "opening" | "source" | "injection" | "roles" | "theme" | "output";
export type Output = "html" | "ansi" | "tokens";

export const tokenRoles = ["Keyword", "String", "Local binding"] as const;

const siteColors: Record<string, string> = {
  "markup.heading.1": "var(--c-code-heading)",
  "markup.raw.block": "var(--c-code-string)",
  label: "var(--c-code-label)",
  keyword: "var(--c-code-keyword)",
  "keyword.function": "var(--c-code-keyword)",
  function: "var(--c-code-function)",
  "function.macro": "var(--c-code-macro)",
  "punctuation.bracket": "var(--c-code-comment)",
  "punctuation.delimiter": "var(--c-code-comment)",
  operator: "var(--c-code-operator)",
  string: "var(--c-code-string)",
};

const siteRainbow = ["var(--c-cpn-red)", "var(--c-cpn-peach)"];

const tokenExcerpt = sampleLines[4]!.filter(([text]) => text === "let" || text === "name" || text === "\"Comfit\"");
const tokenJson = [
  "[",
  ...tokenExcerpt.map(([text, highlight], index) =>
    `  {"text": ${JSON.stringify(text)}, "highlight": ${JSON.stringify(highlight ?? null)}}${
      index < tokenExcerpt.length - 1 ? "," : ""
    }`
  ),
  "]",
].join("\n");

function cssColor(color: unknown) {
  return typeof color === "string" && color.startsWith("#") ? color : undefined;
}

function tokenColor(theme: Theme | undefined, capture: string | undefined, rainbow: number | undefined) {
  const palette = theme?.rainbow;
  if (rainbow !== undefined) {
    return palette?.length ? cssColor(palette[rainbow % palette.length]) : siteRainbow[rainbow];
  }
  if (!capture) return undefined;
  if (!theme) return siteColors[capture];
  return cssColor(theme.styles?.[capture]?.fg) ?? cssColor(theme.default?.fg);
}

function escapeHtml(text: string) {
  return text.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
}

function sampleOutput(output: Exclude<Output, "tokens">, theme: Theme | undefined) {
  const foreground = cssColor(theme?.default?.fg);
  const background = cssColor(theme?.default?.bg);
  const body = sampleLines.map((line) =>
    line.map(([text, capture]) => {
      const color = theme ? tokenColor(theme, capture, undefined) ?? foreground : undefined;

      if (output === "html") {
        const escaped = escapeHtml(text);
        return color && color !== foreground ? `<span style="color:${color}">${escaped}</span>` : escaped;
      }

      const channels = color?.match(/^#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$/i)?.slice(1);
      return channels
        ? `\u001b[38;2;${channels.map((channel) => Number.parseInt(channel, 16)).join(";")}m${text}`
        : text;
    }).join("")
  ).join(output === "ansi" ? "\u001b[0m\n" : "\n");

  if (output === "ansi") return `${body}\u001b[0m`;

  const style = [foreground && `color:${foreground}`, background && `background-color:${background}`].filter(Boolean);
  return `<pre${style.length ? ` style="${style.join(";")}"` : ""}><code>${body}</code></pre>`;
}

interface SampleCodeProps {
  scene?: Scene;
  role?: number;
  rustFocus?: boolean;
  emphasize?: boolean;
  output?: Output;
  theme?: Theme | undefined;
  animatedInk?: boolean;
}

export function SampleCode({
  scene = "opening",
  role = 2,
  rustFocus = false,
  emphasize = false,
  output = "html",
  theme,
  animatedInk = false,
}: SampleCodeProps) {
  const container = useRef<HTMLDivElement>(null);
  const isOutput = scene === "output";
  const title = isOutput ? { html: "HTML preview", ansi: "ANSI preview", tokens: "Token excerpt" }[output] : "Markdown";
  const neutral = scene === "source" || scene === "injection" && !rustFocus;
  const reducedMotion = useReducedMotion();
  const { ink } = useSpring({
    ink: neutral ? 0 : 100,
    immediate: !animatedInk || reducedMotion !== false,
    config: springConfig,
  });
  const themeStyle = {
    "--c-bg": cssColor(theme?.default?.bg),
    "--c-text": cssColor(theme?.default?.fg),
    "--c-text-muted": cssColor(theme?.styles?.comment?.fg),
    "--c-bg-hover": cssColor(theme?.lineHighlight?.bg),
    "--c-bg-muted": cssColor(theme?.lineHighlight?.bg),
    "--c-border": cssColor(theme?.lineHighlight?.bg),
    "--c-button-bg": cssColor(theme?.default?.bg),
    "--c-button-bg-hover": cssColor(theme?.lineHighlight?.bg),
    "--c-button-bg-active": theme ? "color-mix(in oklab, var(--c-text) 14%, var(--c-bg))" : undefined,
    "--c-button-text": cssColor(theme?.styles?.comment?.fg),
    "--c-button-text-hover": cssColor(theme?.default?.fg),
    "--c-code-number": cssColor(theme?.styles?.comment?.fg),
  } as CSSProperties;
  const panelStyle = { ...themeStyle, "--sample-ink": ink.to((value) => `${value}%`) };

  if (isOutput && output === "tokens") {
    return (
      <div className={styles.example} data-scene="output" data-output="tokens">
        <CodeBlock data-code={tokenJson} data-title={title} data-language="json" className={styles.code}>
          <code>{tokenJson}</code>
        </CodeBlock>
      </div>
    );
  }

  return (
    <animated.div
      ref={container}
      className={styles.example}
      data-scene={scene}
      data-output={isOutput ? output : undefined}
      data-rust-focus={rustFocus || undefined}
      style={panelStyle}
    >
      <CodeBlock
        data-code={isOutput && output !== "tokens" ? sampleOutput(output, theme) : sampleSource}
        data-title={title}
        data-language={scene === "opening" || rustFocus ? "rust" : isOutput ? "markdown" : ""}
        className={styles.code}
      >
        <code>
          {sampleLines.map((line, lineIndex) => (
            <Fragment key={lineIndex}>
              {lineIndex > 0 && "\n"}
              <span
                className={emphasize && lineIndex === 4 ? "syntax-line syntax-line-highlight" : "syntax-line"}
                data-region={lineIndex === 0
                  ? "heading"
                  : lineIndex === 2
                  ? "fence-start"
                  : lineIndex === 8
                  ? "fence-end"
                  : lineIndex > 2
                  ? "fence"
                  : undefined}
                data-embedded={lineIndex > 2 && lineIndex < 8 || undefined}
              >
                {line.map(([text, capture, rainbow], tokenIndex) => {
                  const syntax = capture?.split(".").map((_, index, parts) =>
                    `syntax-${parts.slice(0, index + 1).join("-")}`
                  ).join(" ");
                  const bracket = scene === "opening" || emphasize ? rainbow : undefined;
                  const classes = bracket === undefined ? syntax : `${syntax} syntax-rainbow-${bracket}`;
                  const tokenRole = text === "let" ? 0 : text === "\"Comfit\"" ? 1 : text === "name" ? 2 : undefined;
                  const focused = scene === "roles" && tokenRole === role;
                  const muted = scene === "roles" && tokenRole !== undefined && tokenRole > role;
                  return (
                    <span
                      className={classes}
                      data-focused={focused || undefined}
                      data-muted={muted || undefined}
                      style={{ "--token-ink": tokenColor(theme, capture, bracket) } as CSSProperties}
                      key={tokenIndex}
                    >
                      {text}
                    </span>
                  );
                })}
              </span>
            </Fragment>
          ))}
        </code>
      </CodeBlock>
      {scene === "roles" && (
        <TokenCallout container={container} label={tokenRoles[role]!} role={role} animate={animatedInk} />
      )}
    </animated.div>
  );
}
