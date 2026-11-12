/**
 * Options, themes, and highlighting results for the WASM package.
 * @packageDocumentation
 */

/** A URL to fetch or WASM bytes. Importing a descriptor does not fetch its asset. */
export type WasmSource = string | URL | Uint8Array | ArrayBuffer;

/** A language's parser asset, ID, aliases, and Tree-sitter queries. */
export interface Language {
  /** Canonical language ID, unique within one highlighter. */
  readonly id: string;
  /** Additional names accepted by the highlighter. Aliases and IDs share one namespace. */
  readonly aliases?: readonly string[];
  /** Parser WASM URL or bytes. Languages can share the same parser asset. */
  readonly wasm: WasmSource;
  /** Token capture query. An empty string disables this pass. */
  readonly highlights: string;
  /** Embedded-language query. Omitted or empty queries disable this pass. */
  readonly injections?: string;
  /** Lexical scope, definition, and reference query. */
  readonly locals?: string;
  /** Query for brackets and nesting scopes, used to calculate bracket depths. */
  readonly rainbows?: string;
}

/** Options for line emphasis and bracket-depth collection. */
export interface HighlightOptions {
  /** One-based source lines to emphasize. Defaults to an empty list. */
  highlightedLines?: readonly number[];
  /** Collect bracket depths. Defaults to true. */
  rainbow?: boolean;
}

/** A #RGB/#RRGGBB string, "default", or a terminal palette index from 0 through 255. */
export type ThemeColor = string | number;

/** Font and visibility modifiers accepted by the shared theme renderer. */
export type ThemeModifier =
  | "bold"
  | "dim"
  | "italic"
  | "slow_blink"
  | "rapid_blink"
  | "reversed"
  | "hidden"
  | "crossed_out";

/** Optional colors, font modifiers, and underline settings for one capture. */
export interface ThemeStyle {
  /** Foreground color. Inherits the parent value if omitted. */
  readonly fg?: ThemeColor;
  /** Background color. Inherits the parent value if omitted. */
  readonly bg?: ThemeColor;
  /** An omitted or empty list clears inherited modifiers on matching captures. */
  readonly modifiers?: readonly ThemeModifier[];
  /** Underline settings. Inherits the parent settings if omitted. */
  readonly underline?: {
    /** Underline color. Uses the rendering default if omitted. */
    readonly color?: ThemeColor;
    /** Underline shape. Uses a straight line if omitted. */
    readonly style?: "line" | "curl" | "dashed" | "dotted" | "double_line";
  };
}

/** Theme data supplied directly to the WASM engine for validation. */
export interface Theme {
  /** Display name. Named-theme CSS properties use the keys in the theme map instead. */
  readonly name?: string;
  /** Base style inherited by tokens. */
  readonly default?: ThemeStyle;
  /** Capture styles keyed by dotted query capture names. */
  readonly styles?: Readonly<Record<string, ThemeStyle>>;
  /** Bracket colors wrapped by depth. An empty palette disables theme bracket colors. */
  readonly rainbow?: readonly ThemeColor[];
  /** Style applied to source lines selected for emphasis. */
  readonly lineHighlight?: ThemeStyle;
}

/** Element classes and rainbow wrapping shared by all HTML style modes. */
export interface HtmlOptions extends HighlightOptions {
  /** Outer tag name. Defaults to "pre". */
  preTag?: string;
  /** Line tag name. Defaults to "span". */
  lineTag?: string;
  /** Optional class on the outer element, escaped as attribute text. */
  preClass?: string;
  /** Prefix for token, rainbow, and line classes. Defaults to an empty string. */
  classPrefix?: string;
  /** Wrap class/variable depths at this count. Zero preserves depth. Inline themes use their palettes. */
  rainbowColors?: number;
}

/** HTML style mode and its permitted single-theme or named-theme inputs. */
export type RenderHtmlOptions =
  & HtmlOptions
  & (
    | {
      /** Emit capture classes by default, or theme-independent CSS variables. */
      style?: "class" | "variables";
      /** Theme values are not accepted in class or variable mode. */
      theme?: never;
      /** Named themes are not accepted in class or variable mode. */
      themes?: never;
      /** Default-theme selection is only available with named inline themes. */
      defaultTheme?: never;
    }
    | {
      /** Embed theme styles directly in HTML elements. */
      style: "inline";
      /** Single inline theme, mutually exclusive with a named-theme map. */
      theme: Theme;
      /** Named themes cannot be combined with a single theme. */
      themes?: never;
      /** A single theme has no named default to select. */
      defaultTheme?: never;
    }
    | {
      /** Embed the default palette and CSS custom properties for alternative themes. */
      style: "inline";
      /** A single theme cannot be combined with named themes. */
      theme?: never;
      /** Nonempty map of named themes. Keys must be valid CSS theme names that do not collide. */
      themes: Readonly<Record<string, Theme>>;
      /** Required for multiple themes. Must name a supplied theme. A single theme selects itself. */
      defaultTheme?: string;
    }
  );

/** Owned source text with an optional capture style and bracket depth. */
export interface Token {
  /** Source text without newline separators. */
  readonly text: string;
  /** Query capture name, or null for unstyled text. */
  readonly highlight: string | null;
  /** Zero-based bracket depth, or null when no bracket depth was collected. */
  readonly rainbow: number | null;
}

/** A source line without its newline separator, including empty source lines. */
export interface Line {
  /** One-based source line number. */
  readonly number: number;
  /** Whether this line is selected for emphasis. */
  readonly highlighted: boolean;
  /** Tokens in source order. Token boundaries may differ from syntax node boundaries. */
  readonly tokens: readonly Token[];
}

/** Initial source and a loaded language ID or alias. */
export interface CreateDocumentOptions {
  language: string;
  source: string;
}

/** An owned snapshot, independent of document edits or registry disposal. */
export interface HighlightResult {
  /** Number of source lines, without exporting tokens from WASM. */
  readonly lineCount: number;
  /** Exported on first access. Read before disposal. Cached lines remain usable afterward. */
  readonly lines: readonly Line[];
  /** Render without parsing. Rainbow depths are fixed by document.highlight(). */
  renderHtml(options?: RenderHtmlOptions): string;
  /** Release the WASM snapshot. Previously accessed lines remain usable. Repeated calls are harmless. */
  dispose(): void;
}

/** A source document that keeps its syntax trees and cached highlighting. */
export interface Document {
  readonly source: string;
  /** Incrementally parse changed source. If the source is unchanged, do nothing. */
  update(source: string): void;
  /** Return an independent snapshot. Dispose each snapshot when no longer needed. */
  highlight(options?: HighlightOptions): HighlightResult;
  /** Release the document. Existing snapshots remain usable. Repeated calls are harmless. */
  dispose(): void;
}

/** A parser and language registry. Call dispose() when no longer needed. */
export interface Highlighter {
  /** Create an independent document sharing the currently loaded language queries. */
  createDocument(options: CreateDocumentOptions): Document;
  /**
   * Highlight source into owned lines and tokens using a registered ID or alias.
   * Empty sources produce one line. A trailing newline produces a final empty line.
   * @throws If the registry is disposed, options are invalid, or highlighting fails.
   */
  highlight(source: string, language: string, options?: HighlightOptions): readonly Line[];
  /**
   * Render an escaped HTML fragment with capture classes, inline themes, or CSS variables.
   * @throws If the registry is disposed, options are invalid, or highlighting fails.
   */
  renderHtml(source: string, language: string, options?: RenderHtmlOptions): string;
  /** Release runtime resources. Repeated calls are harmless. Later rendering calls throw. */
  dispose(): void;
}

/** WASM sources for initializing both runtimes before creating a highlighter. */
export interface InitializeOptions {
  /** Comfit's shared highlighting engine, built as `comfit_bg.wasm`. */
  engine: WasmSource;
  /** The pinned web-tree-sitter runtime WASM. */
  treeSitter: WasmSource;
}

/** Languages to register while creating an independent highlighter instance. */
export interface CreateHighlighterOptions {
  /** Grammar assets and queries. IDs and aliases must be unique across the list. */
  languages: readonly Language[];
}
