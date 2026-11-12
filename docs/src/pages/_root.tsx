import { COLOR_MODE_SCRIPT } from "#/lib/color-mode";
import type { ReactNode } from "react";

export default function RootElement({ children }: { children: ReactNode }) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link
          rel="icon"
          type="image/svg+xml"
          href="/logos/comfit-icon-light.svg"
          media="(prefers-color-scheme: light)"
        />
        <link rel="icon" type="image/svg+xml" href="/logos/comfit-icon-dark.svg" media="(prefers-color-scheme: dark)" />
        <script dangerouslySetInnerHTML={{ __html: COLOR_MODE_SCRIPT }} />
      </head>
      <body>
        <div id="root">{children}</div>
      </body>
    </html>
  );
}

export const getConfig = async () => {
  return {
    render: "static",
  } as const;
};
