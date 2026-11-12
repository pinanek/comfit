import type { Element, Root } from "hast";
import { visit } from "unist-util-visit";

export default function rehypeFigure() {
  return (tree: Root) => {
    visit(tree, "element", (node) => {
      if (node.tagName !== "p") return;

      const children = node.children.filter((child) => child.type !== "text" || child.value.trim());
      if (children.length !== 1 || children[0]?.type !== "element") return;
      let image = children[0];
      if (image.tagName === "a" && image.children.length === 1) {
        const child = image.children[0];
        if (child?.type !== "element") return;
        image = child;
      }
      if (image.tagName !== "img") return;

      node.tagName = "figure";
      node.properties["data-image-figure"] = true;
      const title = image.properties.title;
      if (typeof title === "string" && title) {
        const caption: Element = {
          type: "element",
          tagName: "figcaption",
          properties: {},
          children: [{ type: "text", value: title }],
        };
        node.children.push(caption);
        delete image.properties.title;
      }
    });
  };
}
