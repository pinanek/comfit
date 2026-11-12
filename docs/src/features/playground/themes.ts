import type { Theme, ThemeColor } from "@comfit/wasm";
import { parse } from "smol-toml";

export interface PlaygroundTheme {
  id: string;
  theme: Theme;
}

const sources = import.meta.glob<string>(
  "../../../../crates/comfit_themes/data/*.toml",
  { eager: true, query: "?raw", import: "default" },
);
const lineColors: Record<string, string> = {
  "catppina-dark": "#29292b",
  "catppina-light": "#e7edf1",
  "catppuccin-mocha": "#313244",
  "catppuccin-latte": "#e6e9ef",
};

export const playgroundThemes: PlaygroundTheme[] = Object.entries(sources).map(([path, source]) => {
  const id = path.split("/").at(-1)!.replace(".toml", "");
  // TOML tables have null prototypes; RSC props require plain objects.
  const theme = JSON.parse(JSON.stringify(parse(source))) as Theme;

  // Use capture colors when the bundled theme has no bracket palette.
  const rainbow = theme.rainbow ?? ["constant.builtin", "number", "type", "string", "function", "keyword"]
    .map((capture) => theme.styles?.[capture]?.fg)
    .filter((color): color is ThemeColor => color !== undefined);
  const lineColor = lineColors[id];

  return {
    id,
    theme: { ...theme, rainbow, ...(lineColor ? { lineHighlight: { bg: lineColor } } : {}) },
  };
});
