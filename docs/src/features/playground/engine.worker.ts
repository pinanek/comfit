import { createHighlighter, type Document, type HighlightResult, initialize, type Theme } from "@comfit/wasm";
import engine from "@comfit/wasm/comfit_bg.wasm?url";
import treeSitter from "@comfit/wasm/web-tree-sitter.wasm?url";
import type { LanguageId } from "./languages";

export interface HighlightRequest {
  id: number;
  source: string;
  language: LanguageId;
  theme: Theme | undefined;
  rainbow: boolean;
  highlightedLines: number[];
}

export type HighlightResponse =
  | { type: "highlight"; id: number; source: string; language: LanguageId; html: string }
  | { type: "output"; id: number; html: string; tokens: string; lineCount: number }
  | { type: "error"; id: number; error: string };

const loaders = {
  rust: () => import("@comfit/wasm-grammars/rust"),
  javascript: () => import("@comfit/wasm-grammars/javascript"),
  typescript: () => import("@comfit/wasm-grammars/typescript"),
  python: () => import("@comfit/wasm-grammars/python"),
  markdown: () => import("@comfit/wasm-grammars/markdown"),
  html: () => import("@comfit/wasm-grammars/html"),
  bash: () => import("@comfit/wasm-grammars/bash"),
};
const documents = new Map<LanguageId, Promise<Document>>();
let latestRequest = 0;
let queued: HighlightRequest | null = null;
let running = false;
let inspection: { timer: ReturnType<typeof setTimeout>; result: HighlightResult } | null = null;

function documentFor(language: LanguageId): Promise<Document> {
  let pending = documents.get(language);
  if (!pending) {
    pending = (async () => {
      await initialize({ engine, treeSitter });
      const languages: LanguageId[] = language === "markdown"
        ? ["markdown", "rust", "javascript", "typescript", "python", "html", "bash"]
        : language === "html" || language === "bash"
        ? ["html", "bash", "javascript", "typescript", "python"]
        : [language];
      const definitions = await Promise.all([
        ...languages.map((id) => loaders[id]()),
        ...(language === "markdown" ? [import("@comfit/wasm-grammars/markdown-inline")] : []),
      ]);

      const registry = await createHighlighter({ languages: definitions.map(({ default: definition }) => definition) });
      try {
        return registry.createDocument({ language, source: "" });
      } finally {
        registry.dispose();
      }
    })().catch((error: unknown) => {
      documents.delete(language);
      throw error;
    });
    documents.set(language, pending);
  }
  return pending;
}

async function highlight(request: HighlightRequest): Promise<void> {
  try {
    const document = await documentFor(request.language);
    if (request.id !== latestRequest) return;

    document.update(request.source);
    const options = { rainbow: request.rainbow, highlightedLines: request.highlightedLines };
    const result = document.highlight(options);
    try {
      const html = result.renderHtml(
        request.theme
          ? { style: "inline", theme: request.theme }
          : { classPrefix: "syntax-", rainbowColors: 6 },
      );
      self.postMessage(
        {
          type: "highlight",
          id: request.id,
          source: request.source,
          language: request.language,
          html,
        } satisfies HighlightResponse,
      );

      // Let the editor paint before formatting and displaying inspection output.
      inspection = {
        result,
        timer: setTimeout(() => {
          inspection = null;
          try {
            if (request.id !== latestRequest) return;
            self.postMessage(
              {
                type: "output",
                id: request.id,
                html,
                tokens: JSON.stringify(result.lines, null, 2),
                lineCount: result.lineCount,
              } satisfies HighlightResponse,
            );
          } catch (error) {
            reportError(request.id, error);
          } finally {
            result.dispose();
          }
        }, 150),
      };
    } catch (error) {
      result.dispose();
      throw error;
    }
  } catch (error) {
    reportError(request.id, error);
  }
}

function reportError(id: number, error: unknown): void {
  if (id !== latestRequest) return;
  self.postMessage(
    { type: "error", id, error: error instanceof Error ? error.message : String(error) } satisfies HighlightResponse,
  );
}

function scheduleHighlight(): void {
  if (running || !queued) return;
  running = true;
  // Yield to message delivery so queued edits collapse to the newest source.
  setTimeout(() => {
    const request = queued!;
    queued = null;
    void highlight(request).finally(() => {
      running = false;
      scheduleHighlight();
    });
  }, 0);
}

self.addEventListener("message", (event: MessageEvent<HighlightRequest>) => {
  latestRequest = event.data.id;
  queued = event.data;
  if (inspection) {
    clearTimeout(inspection.timer);
    inspection.result.dispose();
    inspection = null;
  }
  scheduleHighlight();
});
