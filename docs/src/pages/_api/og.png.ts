import { site } from "#/site";
import { Buffer } from "node:buffer";
import { createElement } from "react";
import { render } from "takumi-js";
import { googleFonts } from "takumi-js/helpers";
import logo from "../../../../logo/comfit-logo-light.svg?raw";
import palette from "../../styles/theme/palette.css?raw";

// Keep the image in the site's light palette regardless of the visitor's theme.
const colors = Object.fromEntries(
  palette.split(":root.dark")[0]!.matchAll(/--c-cpn-([\w]+):\s*(#[\da-f]+);/g).map(
    ([, name, value]) => [name, value],
  ),
);

async function getImage(): Promise<Response> {
  const png = await render(
    createElement(
      "div",
      {
        style: {
          display: "flex",
          flexDirection: "column",
          width: "100%",
          height: "100%",
          padding: "48px 64px",
          backgroundColor: colors.base,
          color: colors.text,
          fontFamily: "Hanken Grotesk",
          fontWeight: 500,
        },
      },
      createElement("img", {
        src: `data:image/svg+xml;base64,${Buffer.from(logo).toString("base64")}`,
        width: 306,
        height: 98,
      }),
      createElement(
        "div",
        {
          style: {
            display: "flex",
            flexDirection: "column",
            marginTop: 36,
            fontSize: 76,
            fontWeight: 700,
            lineHeight: 1.1,
            letterSpacing: "-0.025em",
          },
        },
        createElement("div", null, "Code has structure."),
        createElement("div", { style: { color: colors.blue } }, "Make it visible."),
      ),
      createElement(
        "div",
        { style: { marginTop: 28, fontSize: 30, color: colors.subtext0 } },
        site.description,
      ),
      createElement(
        "div",
        {
          style: {
            display: "flex",
            justifyContent: "space-between",
            position: "absolute",
            bottom: 48,
            left: 64,
            right: 64,
            paddingTop: 20,
            borderTop: `1px solid ${colors.surface1}`,
            fontSize: 24,
            color: colors.subtext0,
          },
        },
        createElement("div", null, "Tokens, HTML, and terminal output"),
        createElement("div", { style: { color: colors.blue } }, new URL(site.url).host),
      ),
    ),
    {
      width: 1200,
      height: 630,
      format: "png",
      fonts: googleFonts([{ name: "Hanken Grotesk", weight: [500, 700] }]),
    },
  );

  return new Response(new Uint8Array(png), {
    headers: {
      "Content-Type": "image/png",
    },
  });
}

// A lowercase declaration keeps Vite's RSC CSS transform from treating GET as a component.
export { getImage as GET };

export const getConfig = async () => {
  return { render: "static" } as const;
};
