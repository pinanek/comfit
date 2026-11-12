/**
 * Initialize the WASM runtimes and manage browser highlighter instances.
 * @packageDocumentation
 */

import init, { create_highlighter, initialize as initializeRuntime } from "./generated/comfit.js";
import type {
  CreateDocumentOptions,
  CreateHighlighterOptions,
  Document,
  Highlighter,
  HighlightOptions,
  HighlightResult,
  InitializeOptions,
  Line,
  RenderHtmlOptions,
  WasmSource,
} from "./types.js";

export type {
  CreateDocumentOptions,
  CreateHighlighterOptions,
  Document,
  Highlighter,
  HighlightOptions,
  HighlightResult,
  HtmlOptions,
  InitializeOptions,
  Language,
  Line,
  RenderHtmlOptions,
  Theme,
  ThemeColor,
  ThemeModifier,
  ThemeStyle,
  Token,
  WasmSource,
} from "./types.js";

let initialization: Promise<void> | undefined;
let initialized = false;

async function loadWasm(source: WasmSource): Promise<Uint8Array<ArrayBuffer>> {
  if (source instanceof Uint8Array || source instanceof ArrayBuffer) {
    return new Uint8Array(source);
  }

  if (typeof source !== "string" && !(source instanceof URL)) {
    throw new TypeError("Expected a WASM URL, Uint8Array, or ArrayBuffer");
  }

  const response = await fetch(source);

  if (!response.ok) {
    throw new Error(`Failed to load ${source}: ${response.status}`);
  }

  return new Uint8Array(await response.arrayBuffer());
}

/**
 * Initialize Comfit and web-tree-sitter from WASM URLs or bytes.
 *
 * Concurrent calls share the first attempt. Successful initialization is reused.
 * A failed attempt can be retried with new sources.
 * @throws Rejects if a source cannot be fetched or either runtime fails to initialize.
 */
export async function initialize(options: InitializeOptions): Promise<void> {
  if (initialization) {
    return initialization;
  }

  if (!options?.engine || !options?.treeSitter) {
    throw new Error("initialize() requires engine and treeSitter WASM sources");
  }

  initialization = Promise.all([loadWasm(options.engine), loadWasm(options.treeSitter)])
    .then(async ([engine, treeSitter]) => {
      await init({ module_or_path: engine });
      await initializeRuntime(treeSitter);
      initialized = true;
    })
    .catch(error => {
      initialization = undefined;
      throw error;
    });

  return initialization;
}

/**
 * Create a highlighter and load its grammars and queries.
 *
 * Await {@link initialize} first. If creation fails, partially loaded resources are released.
 * Call {@link Highlighter.dispose} when the highlighter is no longer needed.
 * @throws Rejects before initialization, for failed asset loads, or for invalid language definitions.
 */
export async function createHighlighter(config: CreateHighlighterOptions): Promise<Highlighter> {
  if (!initialized) {
    throw new Error("Call and await initialize() before createHighlighter()");
  }

  const engine = create_highlighter();
  const downloads = new Map<WasmSource, Promise<Uint8Array>>();

  try {
    const results = await Promise.allSettled(config.languages.map(async language => {
      const key = language.wasm instanceof URL ? language.wasm.href : language.wasm;
      let download = downloads.get(key);

      if (!download) {
        download = loadWasm(language.wasm);
        downloads.set(key, download);
      }

      const { wasm: _, ...definition } = language;
      await engine.load_language(await download, JSON.stringify(definition));
    }));
    const failure = results.find(result => result.status === "rejected");

    if (failure) throw failure.reason;
  } catch (error) {
    engine.free();
    throw error;
  }

  let disposed = false;

  const check = (): void => {
    if (disposed) throw new Error("Highlighter has been disposed");
  };

  return {
    createDocument(documentOptions: CreateDocumentOptions): Document {
      check();
      const document = engine.create_document(documentOptions.language, documentOptions.source);
      let documentDisposed = false;
      const checkDocument = (): void => {
        if (documentDisposed) throw new Error("Document has been disposed");
      };

      return {
        get source(): string {
          checkDocument();
          return document.source();
        },
        update(source: string): void {
          checkDocument();
          document.update(source);
        },
        highlight(highlightOptions: HighlightOptions = {}): HighlightResult {
          checkDocument();
          const result = document.highlight(JSON.stringify(highlightOptions));
          let lines: readonly Line[] | undefined;
          const lineCount = result.line_count();
          let resultDisposed = false;
          return {
            lineCount,
            get lines(): readonly Line[] {
              if (lines) return lines;
              if (resultDisposed) throw new Error("Highlight result has been disposed");
              lines = JSON.parse(result.lines_json()) as Line[];
              return lines;
            },
            renderHtml(options: RenderHtmlOptions = {}): string {
              if (resultDisposed) throw new Error("Highlight result has been disposed");
              return result.render_html(JSON.stringify(options));
            },
            dispose(): void {
              if (!resultDisposed) {
                result.free();
                resultDisposed = true;
              }
            },
          };
        },
        dispose(): void {
          if (!documentDisposed) {
            document.free();
            documentDisposed = true;
          }
        },
      };
    },
    highlight(source: string, language: string, options: HighlightOptions = {}): readonly Line[] {
      check();

      return JSON.parse(engine.highlight_json(source, language, JSON.stringify(options))) as Line[];
    },
    renderHtml(source: string, language: string, options: RenderHtmlOptions = {}): string {
      check();

      return engine.render_html(source, language, JSON.stringify(options));
    },
    dispose(): void {
      if (!disposed) {
        engine.free();
        disposed = true;
      }
    },
  };
}
