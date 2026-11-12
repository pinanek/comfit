import { Document } from "#/features/documentation/document";
import { getStaticPaths } from "#/features/documentation/documents";
import type { PageProps } from "waku/router";

export default function DocsContentPage({ slug }: PageProps<"/docs/[...slug]">) {
  return <Document slug={slug} />;
}

export const getConfig = async () => {
  return {
    render: "static",
    staticPaths: getStaticPaths(),
  } as const;
};
