"use client";

import { Search } from "#/features/search/search";
import { useCloseOnDesktop } from "#/hooks/use-close-on-desktop";
import { Dialog } from "@base-ui/react/dialog";
import { ArrowUpRight, Menu, X } from "lucide-react";
import { useState } from "react";
import { ColorModeButton } from "./color-mode-button";
import { GitHubIcon } from "./icons/github";
import { Logo } from "./logo";
import { Link } from "./navigation-link";
import styles from "./site-nav.module.css";

const links = [
  { href: "/docs", label: "Documentation" },
  { href: "/api-references", label: "API references" },
  { href: "/playground", label: "Playground" },
];

function GitHubLink({ iconOnly = false }: { iconOnly?: boolean }) {
  return (
    <a
      className={styles.link}
      href="https://github.com/pinanek/comfit"
      target="_blank"
      rel="noopener noreferrer"
      aria-label="Comfit on GitHub (opens in a new tab)"
      title="GitHub"
    >
      {iconOnly ? <GitHubIcon width={20} height={20} /> : (
        <>
          GitHub <ArrowUpRight size={12} aria-hidden="true" />
        </>
      )}
    </a>
  );
}

export function SiteNav() {
  const [open, setOpen] = useState(false);

  useCloseOnDesktop(open, () => setOpen(false));

  return (
    <header className={styles.header}>
      <nav className={styles.nav} aria-label="Main">
        <Link className={styles.brand} to="/" aria-label="Comfit home">
          <Logo className={styles.logo} />
        </Link>
        <div className={styles.desktopLinks}>
          {links.map(({ href, label }) => <Link key={href} className={styles.link} to={href}>{label}</Link>)}
        </div>
        <div className={styles.links}>
          <Search />
          <div className={styles.desktopLinks}>
            <GitHubLink iconOnly />
            <ColorModeButton />
          </div>
          <Dialog.Root open={open} onOpenChange={setOpen}>
            <Dialog.Trigger className={styles.menuButton} aria-label="Open main menu">
              <Menu size={20} aria-hidden="true" />
            </Dialog.Trigger>
            <Dialog.Portal>
              <Dialog.Backdrop className={styles.backdrop} />
              <Dialog.Popup className={styles.popup}>
                <div className={styles.popupHeader}>
                  <Dialog.Title className={styles.popupTitle}>Comfit</Dialog.Title>
                  <Dialog.Close className={styles.closeButton} aria-label="Close main menu">
                    <X size={20} aria-hidden="true" />
                  </Dialog.Close>
                </div>
                <nav className={styles.menuLinks} aria-label="Main menu">
                  {links.map(({ href, label }) => (
                    <Link
                      key={href}
                      className={styles.link}
                      to={href}
                      onClick={() => setOpen(false)}
                    >
                      {label}
                    </Link>
                  ))}
                  <GitHubLink />
                </nav>
                <div className={styles.menuFooter}>
                  <span>Color theme</span>
                  <ColorModeButton />
                </div>
              </Dialog.Popup>
            </Dialog.Portal>
          </Dialog.Root>
        </div>
      </nav>
    </header>
  );
}
