import { valueToEstree } from "estree-util-value-to-estree";
import type { Root } from "hast";
import { toString } from "hast-util-to-string";
import type { MdxjsEsmHast } from "mdast-util-mdxjs-esm";
import { visit } from "unist-util-visit";
import type { TocItem } from "../src/features/documentation/types";

export default function rehypeToc() {
  return (tree: Root) => {
    const items: TocItem[] = [];

    visit(tree, "element", (node) => {
      if (!/^h[2-4]$/.test(node.tagName) || typeof node.properties.id !== "string") return;

      items.push({
        id: node.properties.id,
        title: toString(node),
        depth: Number(node.tagName[1]),
      });
    });

    const declaration: MdxjsEsmHast = {
      type: "mdxjsEsm",
      value: "",
      data: {
        estree: {
          type: "Program",
          sourceType: "module",
          body: [{
            type: "ExportNamedDeclaration",
            specifiers: [],
            attributes: [],
            source: null,
            declaration: {
              type: "VariableDeclaration",
              kind: "const",
              declarations: [{
                type: "VariableDeclarator",
                id: { type: "Identifier", name: "toc" },
                init: valueToEstree(items),
              }],
            },
          }],
        },
      },
    };
    tree.children.push(declaration);
  };
}
