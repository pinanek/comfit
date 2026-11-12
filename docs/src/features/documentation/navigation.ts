import apiManifest from "../../../content/api-references/manifest.json";

export type Group = { label: string; items: Item[] };
export type Item = { label: string; href: string } | Group;
export const sections: Group[] = [
  {
    label: "Getting started",
    items: [
      { label: "Overview", href: "/docs" },
      { label: "Node.js", href: "/docs/getting-started/node" },
      { label: "Browser", href: "/docs/getting-started/browser" },
      { label: "Rust", href: "/docs/getting-started/rust" },
      { label: "Python", href: "/docs/getting-started/python" },
      { label: "Command line", href: "/docs/getting-started/cli" },
    ],
  },
  {
    label: "Rendering",
    items: [
      { label: "HTML and CSS", href: "/docs/guides/html" },
      { label: "Themes", href: "/docs/guides/themes" },
      { label: "Lines and tokens", href: "/docs/guides/tokens" },
    ],
  },
  {
    label: "Working with code",
    items: [
      { label: "Languages", href: "/docs/guides/languages" },
      { label: "Embedded languages", href: "/docs/guides/embedded-languages" },
      { label: "Updating source", href: "/docs/guides/documents" },
    ],
  },
  {
    label: "Integrations",
    items: [
      { label: "Markdown and MDX", href: "/docs/guides/markdown" },
    ],
  },
  {
    label: "Help and development",
    items: [
      { label: "Troubleshooting", href: "/docs/guides/troubleshooting" },
      { label: "Build from source", href: "/docs/development/build-from-source" },
    ],
  },
];

export const apiSections: Group[] = [{
  label: "API references",
  items: [...apiManifest.sidebar, { label: "Rust", href: "/api-references/rust" }],
}];
