import { valueToEstree } from "estree-util-value-to-estree";
import type { Element, Root } from "hast";
import type { MdxJsxAttribute } from "mdast-util-mdx-jsx";
import { visit } from "unist-util-visit";
import type { VFile } from "vfile";
import type { CodeTabDefinition } from "../src/mdx/code-tabs";

export default function rehypeCodeTabs() {
  return (tree: Root, file: VFile) => {
    visit(tree, "mdxJsxFlowElement", (node) => {
      if (node.name !== "CodeTabs") return;

      const tabs: CodeTabDefinition[] = [];
      for (const child of node.children) {
        if (child.type === "text" && !child.value.trim()) continue;
        if (child.type !== "mdxJsxFlowElement" || child.name !== "CodeTab") {
          file.fail("CodeTabs must contain CodeTab elements", child);
        }

        const label = child.attributes.find((attribute): attribute is MdxJsxAttribute =>
          attribute.type === "mdxJsxAttribute" && attribute.name === "label"
        )?.value;
        if (typeof label !== "string" || !label.trim()) {
          file.fail("CodeTab requires a nonempty string label", child);
        }
        if (tabs.some((tab) => tab.label === label)) file.fail(`Duplicate CodeTab label: ${label}`, child);

        const pre = child.children.find((node): node is Element => node.type === "element" && node.tagName === "pre");
        const language = pre?.properties["data-language"];
        const title = pre?.properties["data-title"];
        tabs.push({
          label,
          ...(typeof language === "string" ? { language } : {}),
          ...(typeof title === "string" ? { title } : {}),
        });
      }

      if (!tabs.length) file.fail("CodeTabs requires at least one CodeTab", node);
      if (node.attributes.some((attribute) => attribute.type === "mdxJsxAttribute" && attribute.name === "tabs")) {
        file.fail("CodeTabs metadata is generated from CodeTab elements", node);
      }

      // Pass explicit metadata across the RSC boundary without inspecting rendered children.
      node.attributes.push({
        type: "mdxJsxAttribute",
        name: "tabs",
        value: {
          type: "mdxJsxAttributeValueExpression",
          value: JSON.stringify(tabs),
          data: {
            estree: {
              type: "Program",
              sourceType: "module",
              body: [{ type: "ExpressionStatement", expression: valueToEstree(tabs) }],
            },
          },
        },
      });
    });
  };
}
