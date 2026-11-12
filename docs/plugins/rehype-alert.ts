import type { Root } from "hast";
import type { MdxJsxFlowElementHast } from "mdast-util-mdx-jsx";
import { visit } from "unist-util-visit";

export default function rehypeAlert() {
  return (tree: Root) => {
    visit(tree, "element", (node, index, parent) => {
      if (node.tagName !== "blockquote" || index === undefined || !parent) return;

      const paragraph = node.children.find((child) => child.type === "element");
      if (paragraph?.type !== "element" || paragraph.tagName !== "p") return;
      const first = paragraph.children[0];
      if (first?.type !== "text") return;
      const marker = /^\[!(NOTE|TIP|IMPORTANT|WARNING|CAUTION)\](?:\r?\n|$)/i.exec(first.value);
      if (!marker) return;

      first.value = first.value.slice(marker[0].length);
      if (!first.value) paragraph.children.shift();
      if (!paragraph.children.length) node.children.splice(node.children.indexOf(paragraph), 1);

      const alert: MdxJsxFlowElementHast = {
        type: "mdxJsxFlowElement",
        name: "Alert",
        attributes: [{ type: "mdxJsxAttribute", name: "kind", value: marker[1]!.toLowerCase() }],
        children: node.children,
      };
      parent.children[index] = alert;
    });
  };
}
