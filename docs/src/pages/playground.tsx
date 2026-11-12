import { Metadata } from "#/components/metadata";
import { Playground } from "#/features/playground/playground";
import { playgroundThemes } from "#/features/playground/themes";

export default function PlaygroundPage() {
  return (
    <>
      <Metadata
        title="Playground"
        description="Try Comfit syntax highlighting with your own code, languages, and themes."
        path="/playground"
      />
      <Playground themes={playgroundThemes} />
    </>
  );
}

export const getConfig = async () => {
  return {
    render: "static",
  } as const;
};
