/// <reference types="vite/client" />
/// <reference types="@types/mdx" />

declare module "*.css";

declare module "*.md" {
  export const metadata: import("#/components/metadata").MetadataValues;
  export const toc: import("#/features/documentation/types").TocItem[];
  const Content: import("mdx/types").MDXContent;
  export default Content;
}
