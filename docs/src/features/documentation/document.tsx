import { Metadata } from "#/components/metadata";
import { components } from "#/mdx/components";
import proseStyles from "#/mdx/prose.module.css";
import { getDocument, isApiReference, resolveDocumentLink } from "./documents";
import { DocsFooter } from "./footer";
import styles from "./layout.module.css";
import { DocsSidebar } from "./sidebar";
import { DocsToc } from "./toc";

export async function Document({ slug = [] }: { slug?: string[] }) {
  const document = getDocument(slug.join("/"));
  const { default: Content, toc, metadata } = await document.load();
  const apiReferences = isApiReference(document.slug);
  const headings = toc.filter((item) => apiReferences || item.depth <= 3);
  const path = apiReferences ? `/${document.slug}` : `/docs${document.slug ? `/${document.slug}` : ""}`;

  return (
    <>
      <Metadata {...document.metadata} {...metadata} path={path} />
      <DocsSidebar apiReferences={apiReferences} toc={headings} />
      <div className={styles.document} data-has-toc={headings.length > 0 || undefined}>
        <DocsToc key={document.slug} items={headings} />
        <div className={styles.content}>
          <article className={proseStyles.prose} id="docs-content" tabIndex={-1} data-pagefind-body>
            <Content
              components={{
                ...components,
                a: ({ href, ...props }) => <components.a {...props} href={resolveDocumentLink(href, document.file)} />,
              }}
            />
          </article>
          <DocsFooter path={path} />
        </div>
      </div>
    </>
  );
}
