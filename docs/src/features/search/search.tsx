"use client";

import { Kbd } from "#/components/kbd";
import { Link } from "#/components/navigation-link";
import { search, type SearchResult } from "#/features/search/pagefind";
import { useKeyboardShortcuts } from "#/hooks/use-keyboard-shortcuts";
import { Autocomplete } from "@base-ui/react/autocomplete";
import { Dialog } from "@base-ui/react/dialog";
import { ArrowDown, ArrowUp, CornerDownLeft, Search as SearchIcon, X } from "lucide-react";
import { type RefObject, useEffect, useRef, useState } from "react";
import styles from "./search.module.css";

const shortcut = "$mod+KeyK";

type SearchState = { query: string; results: SearchResult[]; status: "ready" | "error" };

function SearchContent(
  { inputRef, onNavigate }: { inputRef: RefObject<HTMLInputElement | null>; onNavigate: () => void },
) {
  const [query, setQuery] = useState("");
  const [attempt, setAttempt] = useState(0);
  const [state, setState] = useState<SearchState>({ query: "", results: [], status: "ready" });
  const term = query.trim();
  const pending = term !== "" && state.query !== term;
  const results = term && !pending && state.status === "ready" ? state.results : [];

  useEffect(() => {
    if (!term) return;

    const controller = new AbortController();
    const timer = window.setTimeout(() => {
      search(term).then(
        (results) => {
          if (!controller.signal.aborted) setState({ query: term, results, status: "ready" });
        },
        () => {
          if (!controller.signal.aborted) setState({ query: term, results: [], status: "error" });
        },
      );
    }, 160);

    return () => {
      controller.abort();
      window.clearTimeout(timer);
    };
  }, [term, attempt]);

  const message = !term
    ? "Search guides and API reference."
    : pending
    ? "Searching…"
    : state.status === "error"
    ? "Search couldn’t load. Try again."
    : results.length === 0
    ? `No results for “${term}”. Try another term.`
    : `${results.length} results`;

  return (
    <Autocomplete.Root
      items={results}
      value={query}
      onValueChange={setQuery}
      itemToStringValue={(item) => item.meta.title}
      filter={null}
      autoHighlight="always"
      inline
      open
    >
      <div className={styles.inputRow}>
        <SearchIcon size={20} aria-hidden="true" />
        <Autocomplete.Input
          ref={inputRef}
          className={styles.input}
          aria-label="Search documentation"
          placeholder="Search documentation…"
          autoComplete="off"
          autoCapitalize="none"
          spellCheck={false}
          enterKeyHint="search"
        />
        <Dialog.Close className={styles.close} aria-label="Close search">
          <X size={20} aria-hidden="true" />
        </Dialog.Close>
      </div>
      <Autocomplete.Status className={results.length > 0 ? styles.srOnly : styles.status}>
        {message}
      </Autocomplete.Status>
      {term && !pending && state.status === "error" && (
        <button
          className={styles.retry}
          onClick={() => {
            setState({ query: "", results: [], status: "ready" });
            setAttempt((attempt) => attempt + 1);
          }}
        >
          Try again
        </button>
      )}
      <Autocomplete.List className={styles.results} aria-label="Search results" aria-busy={pending}>
        {(result: SearchResult) => (
          <Autocomplete.Item
            key={result.url}
            value={result}
            className={styles.result}
            render={(props) => <Link {...props} to={result.url}>{props.children}</Link>}
            onClick={onNavigate}
          >
            <span className={styles.resultTitle}>{result.meta.title}</span>
            <span className={styles.excerpt}>{result.excerpt}</span>
          </Autocomplete.Item>
        )}
      </Autocomplete.List>
      <div className={styles.footer} aria-hidden="true">
        <span>
          <ArrowUp size={12} />
          <ArrowDown size={12} /> Navigate
        </span>
        <span>
          <CornerDownLeft size={12} /> Open
        </span>
        <span className={styles.escape}>
          <Kbd shortcut="Esc" /> Close
        </span>
      </div>
    </Autocomplete.Root>
  );
}

export function Search() {
  const [open, setOpen] = useState(false);
  const trigger = useRef<HTMLButtonElement>(null);
  const input = useRef<HTMLInputElement>(null);

  useKeyboardShortcuts({
    [shortcut]: (event) => {
      event.preventDefault();
      setOpen((open) => !open);
    },
  }, { allowInInputs: true });

  return (
    <Dialog.Root open={open} onOpenChange={setOpen}>
      <Dialog.Trigger
        ref={trigger}
        className={styles.trigger}
        aria-label="Search documentation"
        aria-keyshortcuts="Meta+K Control+K"
      >
        <SearchIcon size={18} aria-hidden="true" />
        <span className={styles.triggerLabel}>Search docs…</span>
        <span className={styles.triggerKeys}>
          <Kbd shortcut={shortcut} />
        </span>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Backdrop className={styles.backdrop} />
        <Dialog.Popup className={styles.popup} initialFocus={input} finalFocus={trigger}>
          <Dialog.Title className={styles.srOnly}>Search documentation</Dialog.Title>
          <SearchContent inputRef={input} onNavigate={() => setOpen(false)} />
        </Dialog.Popup>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
