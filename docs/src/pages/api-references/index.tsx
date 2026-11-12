import { Document } from "#/features/documentation/document";

export default function ApiReferencesPage() {
  return <Document slug={["api-references"]} />;
}

export const getConfig = async () => {
  return {
    render: "static",
  } as const;
};
