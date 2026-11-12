"use client";

import { startTransition, useEffect, useRef, useState } from "react";
import type { HighlightRequest, HighlightResponse } from "./engine.worker";

export function usePlaygroundEngine(
  { source, language, theme, rainbow, highlightedLines }: Omit<HighlightRequest, "id">,
) {
  const [highlighting, setHighlighting] = useState<Extract<HighlightResponse, { type: "highlight" }> | null>(null);
  const [result, setResult] = useState<Exclude<HighlightResponse, { type: "highlight" }> | null>(null);
  const [pending, setPending] = useState<"highlight" | "output" | false>("highlight");
  const [retry, setRetry] = useState(0);
  const workerRef = useRef<Worker | null>(null);
  const requestId = useRef(0);

  useEffect(() => {
    const fail = (error: unknown) => {
      setHighlighting(null);
      setResult({
        type: "error",
        id: requestId.current,
        error: error instanceof Error ? error.message : String(error),
      });
      setPending(false);
    };

    let worker: Worker;
    try {
      worker = new Worker(new URL("./engine.worker.ts", import.meta.url), { type: "module" });
    } catch (error) {
      fail(error);
      return;
    }
    workerRef.current = worker;
    worker.onmessage = ({ data }: MessageEvent<HighlightResponse>) => {
      if (data.id !== requestId.current) return;
      if (data.type === "highlight") {
        setHighlighting(data);
        setResult((previous) => previous?.type === "error" ? null : previous);
        setPending("output");
      } else if (data.type === "error") {
        fail(data.error);
      } else {
        startTransition(() => {
          setResult(data);
          setPending(false);
        });
      }
    };
    worker.onerror = (event) => {
      event.preventDefault();
      worker.terminate();
      workerRef.current = null;
      fail(event.message || "The highlighting worker could not start.");
    };

    return () => {
      worker.terminate();
      workerRef.current = null;
    };
  }, [retry]);

  useEffect(() => {
    if (!workerRef.current) return;
    const id = ++requestId.current;
    if (source.length > 100_000) {
      setResult({ type: "error", id, error: "Please keep the source under 100,000 characters." });
      setPending(false);
      return;
    }

    setPending("highlight");
    const timer = setTimeout(() => {
      const lineCount = source.split("\n").length;
      const request: HighlightRequest = {
        id,
        source,
        language,
        theme,
        rainbow,
        highlightedLines: highlightedLines.filter((line) => line <= lineCount),
      };
      workerRef.current?.postMessage(request);
    }, 25);

    return () => clearTimeout(timer);
  }, [source, language, theme, rainbow, highlightedLines, retry]);

  const output = result?.type === "output" ? result : null;
  const error = result?.type === "error" ? result.error : null;

  return {
    highlighting,
    output,
    error,
    pending,
    retry: () => setRetry((attempt) => attempt + 1),
  };
}
