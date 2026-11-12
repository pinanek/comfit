import type { Element, ElementContent, Root } from "hast";
import { toString } from "hast-util-to-string";
import { SKIP, visit } from "unist-util-visit";

export default function rehypeCodeBlock() {
  return (tree: Root) => {
    visit(tree, "element", (pre) => {
      if (pre.tagName !== "pre") return;

      const code = pre.children.find((child): child is Element => child.type === "element" && child.tagName === "code");
      if (!code) return SKIP;

      const source = toString(code);
      const classes = Array.isArray(code.properties.className) ? code.properties.className.map(String) : [];
      const language = classes.find((name) => name.startsWith("language-"))?.slice("language-".length);
      const meta = typeof code.data?.meta === "string" ? code.data.meta : "";
      const title = /(?:^|\s)title=(?:"([^"]*)"|'([^']*)')(?=\s|$)/.exec(meta);
      pre.properties["data-code"] = source;
      if (language) pre.properties["data-language"] = language;
      if (title) pre.properties["data-title"] = title[1] ?? title[2] ?? "";

      if (code.children.every((child) => child.type === "text")) {
        const children: ElementContent[] = [];
        for (const [index, line] of source.split("\n").entries()) {
          if (index > 0) children.push({ type: "text", value: "\n" });
          children.push({
            type: "element",
            tagName: "span",
            properties: { className: ["syntax-line"] },
            children: line ? [{ type: "text", value: line }] : [],
          });
        }
        code.children = children;
      }

      // A fence's trailing newline is preserved, but needs no extra numbered row.
      const last = code.children.at(-1);
      if (source.endsWith("\n") && code.children.length > 1 && last?.type === "element" && last.children.length === 0) {
        code.children.pop();
      }

      return SKIP;
    });
  };
}
