import { Document } from "#/features/documentation/document";

export default function DocsPage() {
  return <Document />;
}

export const getConfig = async () => {
  return {
    render: "static",
  } as const;
};
