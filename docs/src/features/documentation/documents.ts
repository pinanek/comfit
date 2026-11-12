import type { MetadataValues } from "#/components/metadata";
import type { MDXModule } from "mdx/types";
import apiManifest from "../../../content/api-references/manifest.json";
import type { TocItem } from "./types";

type DocumentModule = MDXModule & { toc: TocItem[]; metadata: MetadataValues };

const apiMetadata: Record<string, MetadataValues> = apiManifest.pages;

const entries = Object.entries(import.meta.glob<DocumentModule>("/content/**/*.md"))
  .map(([file, load]) => ({ file, load, slug: documentSlug(file) }))
  .sort((left, right) => left.slug.localeCompare(right.slug));
const documents = new Map(entries.map((document) => [document.slug, document]));

if (documents.size !== entries.length) {
  throw new Error("Documentation files resolve to duplicate routes");
}

function documentSlug(file: string): string {
  return file.replace(/^\/content\//, "").replace(/(^|\/)index\.md$/, "").replace(/\.md$/, "");
}

export function isApiReference(slug: string): boolean {
  return slug === "api-references" || slug.startsWith("api-references/");
}

export function resolveDocumentLink(href: string | undefined, file: string): string | undefined {
  if (!href || href.startsWith("#")) return href;

  const url = new URL(href, `https://docs.invalid${file}`);
  if (url.origin !== "https://docs.invalid" || !url.pathname.startsWith("/content/") || !url.pathname.endsWith(".md")) {
    return href;
  }

  const slug = documentSlug(url.pathname);
  const route = isApiReference(slug) ? `/${slug}` : `/docs${slug ? `/${slug}` : ""}`;
  return `${route}${url.search}${url.hash}`;
}

export function getStaticPaths(section: "docs" | "api-references" = "docs"): string[][] {
  return entries.flatMap(({ slug }) => {
    if (!slug || slug === "api-references") return [];
    const apiReference = isApiReference(slug);
    if (apiReference !== (section === "api-references")) return [];

    return [(apiReference ? slug.slice("api-references/".length) : slug).split("/")];
  });
}

export function getDocument(path: string) {
  const document = documents.get(path);
  if (!document) throw new Error(`Unknown documentation page: ${path}`);
  return { ...document, metadata: apiMetadata[`/${document.slug}`] };
}
