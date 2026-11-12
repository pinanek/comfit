import { Document } from "#/features/documentation/document";
import { getStaticPaths } from "#/features/documentation/documents";
import type { PageProps } from "waku/router";

export default function ApiReferencePage({ slug }: PageProps<"/api-references/[...slug]">) {
  return <Document slug={["api-references", ...slug]} />;
}

export const getConfig = async () => {
  return {
    render: "static",
    staticPaths: getStaticPaths("api-references"),
  } as const;
};
