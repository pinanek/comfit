import { type Range, RangeSet, StateEffect, StateField, type Text } from "@codemirror/state";
import { Decoration, type DecorationSet, EditorView, gutterLineClass, GutterMarker } from "@codemirror/view";

interface Highlighting {
  decorations: DecorationSet;
  gutters: RangeSet<GutterMarker>;
  style: string;
}

const highlightedLine = new class extends GutterMarker {
  elementClass = "cm-comfit-highlighted";
}();

const emptyHighlighting: Highlighting = { decorations: Decoration.none, gutters: RangeSet.empty, style: "" };

export const setHighlighting = StateEffect.define<Highlighting | null>();
export const comfitHighlighting = StateField.define<Highlighting>({
  create: () => emptyHighlighting,
  update(value, transaction) {
    for (const effect of transaction.effects) {
      if (effect.is(setHighlighting)) return effect.value ?? emptyHighlighting;
    }
    if (!transaction.docChanged) return value;

    return {
      ...value,
      decorations: value.decorations.map(transaction.changes),
      gutters: value.gutters.map(transaction.changes),
    };
  },
  provide: (field) => [
    EditorView.decorations.from(field, (value) => value.decorations),
    gutterLineClass.from(field, (value) => value.gutters),
    EditorView.editorAttributes.from(field, (value) => ({ style: value.style })),
  ],
});

export function highlightingFromHtml(html: string, source: Text): Highlighting {
  // Reuse Comfit's CSS resolution, including capture inheritance and rainbow colors.
  const template = document.createElement("template");
  template.innerHTML = html;
  const pre = template.content.querySelector("pre");
  const style = pre?.style;
  if (style?.color) {
    style.setProperty("--editor-foreground", style.color);
    style.setProperty(
      "--editor-highlight-background",
      `color-mix(in oklab, ${style.color} 8%, ${style.backgroundColor || "var(--c-bg)"})`,
    );
  }
  const lines = template.content.querySelectorAll<HTMLElement>("pre > code > span");
  const decorations: Range<Decoration>[] = [];
  const lineMarkers: Range<GutterMarker>[] = [];

  for (const [index, line] of lines.entries()) {
    const { from } = source.line(index + 1);
    if (line.classList.contains("syntax-line-highlight") || line.classList.contains("line-highlight")) {
      lineMarkers.push(highlightedLine.range(from));
      // Inline rendering uses transparent/inherit for unset line colors.
      const { backgroundColor, color } = line.style;
      if (backgroundColor && backgroundColor !== "transparent" && backgroundColor !== "inherit") {
        style?.setProperty("--editor-highlight-background", backgroundColor);
      }
      if (color && color !== "inherit" && color !== "currentcolor") {
        style?.setProperty("--editor-highlight-foreground", color);
      }
      line.style.removeProperty("background-color");
      line.style.removeProperty("color");
      decorations.push(
        Decoration.line({
          class: "cm-comfit-highlighted",
          attributes: { style: line.style.cssText },
        }).range(from),
      );
    }
    let offset = from;
    for (const token of line.children) {
      // DOM text lengths and CodeMirror positions are both UTF-16 code units.
      const end = offset + (token.textContent?.length ?? 0);
      const style = token.getAttribute("style");
      const classes = token.getAttribute("class");
      if (end > offset && (style || classes)) {
        decorations.push(
          Decoration.mark({
            attributes: { ...(style ? { style } : {}), ...(classes ? { class: classes } : {}) },
          }).range(offset, end),
        );
      }
      offset = end;
    }
  }
  return {
    decorations: Decoration.set(decorations, true),
    gutters: RangeSet.of(lineMarkers),
    style: pre?.getAttribute("style") ?? "",
  };
}
