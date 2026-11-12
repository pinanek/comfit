import "#/styles/index.css";

import { SiteFooter } from "#/components/site-footer";
import { SiteNav } from "#/components/site-nav";
import type { ReactNode } from "react";

type RootLayoutProps = { children: ReactNode };

export default function RootLayout({ children }: RootLayoutProps) {
  return (
    <>
      <SiteNav />
      <main className="site-content">{children}</main>
      <SiteFooter />
    </>
  );
}

export const getConfig = async () => {
  return {
    render: "static",
  } as const;
};
