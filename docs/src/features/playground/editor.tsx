"use client";

import { EditorState } from "@codemirror/state";
import { keymap, lineNumbers } from "@codemirror/view";
import { basicSetup, EditorView } from "codemirror";
import { useEffect, useEffectEvent, useRef } from "react";
import { comfitHighlighting, highlightingFromHtml, setHighlighting } from "./highlighting";
import styles from "./playground.module.css";

export function Editor({ value, onChange, onToggleLine, highlighting }: {
  value: string;
  onChange: (value: string) => void;
  onToggleLine: (line: number) => void;
  highlighting: { source: string; html: string } | null;
}) {
  const host = useRef<HTMLDivElement>(null);
  const view = useRef<EditorView | null>(null);
  const handleChange = useEffectEvent(onChange);
  const toggleLine = useEffectEvent(onToggleLine);

  useEffect(() => {
    if (!host.current) return;

    const editor = new EditorView({
      parent: host.current,
      extensions: [
        basicSetup,
        lineNumbers({
          domEventHandlers: {
            mousedown: (editor, line, event) => {
              if (!(event instanceof MouseEvent) || event.button !== 0) return false;
              toggleLine(editor.state.doc.lineAt(line.from).number);
              return true;
            },
          },
        }),
        keymap.of([{
          key: "Mod-Shift-l",
          run: (editor) => {
            toggleLine(editor.state.doc.lineAt(editor.state.selection.main.head).number);
            return true;
          },
        }]),
        EditorState.tabSize.of(2),
        comfitHighlighting,
        EditorView.contentAttributes.of({
          "aria-label": "Source code",
          "aria-description": "Click line numbers or press Ctrl/Command+Shift+L to toggle line highlighting.",
          "aria-keyshortcuts": "Control+Shift+L Meta+Shift+L",
          spellcheck: "false",
          autocapitalize: "off",
        }),
        EditorView.updateListener.of((update) => {
          if (update.docChanged) handleChange(update.state.doc.toString());
        }),
      ],
    });
    view.current = editor;

    return () => {
      view.current = null;
      editor.destroy();
    };
  }, []);

  useEffect(() => {
    const editor = view.current;
    if (editor && editor.state.doc.toString() !== value) {
      editor.dispatch({ changes: { from: 0, to: editor.state.doc.length, insert: value } });
    }
  }, [value]);

  useEffect(() => {
    const editor = view.current;
    if (!editor) return;
    if (!highlighting) {
      editor.dispatch({ effects: setHighlighting.of(null) });
      return;
    }
    if (highlighting.source !== editor.state.doc.toString()) return;

    editor.dispatch({ effects: setHighlighting.of(highlightingFromHtml(highlighting.html, editor.state.doc)) });
  }, [highlighting, value]);

  return <div ref={host} className={styles.editor} />;
}
