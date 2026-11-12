import type { MDXComponents } from "mdx/types";
import { Alert } from "./alert";
import { CodeBlock } from "./code-block";
import { CodeTab, CodeTabs } from "./code-tabs";
import { Heading } from "./heading";
import { Image } from "./image";
import { MarkdownLink } from "./link";
import { Table } from "./table";

export const components = {
  h1: (props) => <Heading as="h1" {...props} />,
  h2: (props) => <Heading as="h2" {...props} />,
  h3: (props) => <Heading as="h3" {...props} />,
  h4: (props) => <Heading as="h4" {...props} />,
  h5: (props) => <Heading as="h5" {...props} />,
  h6: (props) => <Heading as="h6" {...props} />,
  a: MarkdownLink,
  img: Image,
  Alert,
  pre: CodeBlock,
  table: Table,
  CodeTabs,
  CodeTab,
} satisfies MDXComponents;
