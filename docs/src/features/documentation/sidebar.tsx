"use client";

import { Link } from "#/components/navigation-link";
import type { TocItem } from "#/features/documentation/types";
import { useCloseOnDesktop } from "#/hooks/use-close-on-desktop";
import { isPointerActivation } from "#/lib/pointer-activation";
import { Collapsible } from "@base-ui/react/collapsible";
import { Dialog } from "@base-ui/react/dialog";
import { ChevronRight, Menu, X } from "lucide-react";
import { useState } from "react";
import { useRouter } from "waku";
import { apiSections, type Group, type Item, sections } from "./navigation";
import styles from "./sidebar.module.css";
import { DocsToc } from "./toc";

type ItemProps = {
  item: Item;
  path: string;
  onNavigate?: (() => void) | undefined;
};

function containsPath(item: Item, path: string): boolean {
  return "items" in item
    ? item.items.some((child) => containsPath(child, path))
    : path === item.href
      || (item.href !== "/docs" && item.href !== "/api-references" && path.startsWith(`${item.href}/`));
}

function SidebarGroup({ item, path, onNavigate }: ItemProps & { item: Group }) {
  const active = containsPath(item, path);
  const [state, setState] = useState({ path, open: active });

  if (state.path !== path) {
    setState({ path, open: active || state.open });
  }

  return (
    <Collapsible.Root open={state.open} onOpenChange={(open) => setState({ path, open })}>
      <Collapsible.Trigger className={styles.trigger} data-active={active || undefined}>
        <ChevronRight className={styles.chevron} size={14} aria-hidden="true" />
        {item.label}
      </Collapsible.Trigger>
      <Collapsible.Panel hiddenUntilFound>
        <ul className={styles.nested}>
          {item.items.map((child) => (
            <SidebarItem
              key={child.label}
              item={child}
              path={path}
              onNavigate={onNavigate}
            />
          ))}
        </ul>
      </Collapsible.Panel>
    </Collapsible.Root>
  );
}

function SidebarItem({ item, path, onNavigate }: ItemProps) {
  return (
    <li>
      {"items" in item ? <SidebarGroup item={item} path={path} onNavigate={onNavigate} /> : (
        <Link
          to={item.href}
          className={styles.link}
          aria-current={path === item.href ? "page" : undefined}
          data-active={containsPath(item, path) || undefined}
          onClick={onNavigate}
        >
          {item.label}
        </Link>
      )}
    </li>
  );
}

function SidebarNavigation({ apiReferences, path, onNavigate }: {
  apiReferences: boolean;
  path: string;
  onNavigate?: () => void;
}) {
  const navigationSections = apiReferences ? apiSections : sections;

  return (
    <nav className={styles.navigation} aria-label={apiReferences ? "API references" : "Documentation"}>
      {apiReferences && (
        <ul className={styles.list}>
          <SidebarItem
            item={{ label: "Overview", href: "/api-references" }}
            path={path}
            onNavigate={onNavigate}
          />
        </ul>
      )}
      {navigationSections.map((section) => (
        <section className={styles.section} key={section.label}>
          <h2 className={styles.heading}>{section.label}</h2>
          <ul className={styles.list}>
            {section.items.map((item) => (
              <SidebarItem key={item.label} item={item} path={path} onNavigate={onNavigate} />
            ))}
          </ul>
        </section>
      ))}
    </nav>
  );
}

export function DocsSidebar({ apiReferences = false, toc }: { apiReferences?: boolean; toc: TocItem[] }) {
  const { path } = useRouter();
  const currentPath = path.replace(/\/$/, "");
  const title = apiReferences ? "API references" : "Documentation";
  const [open, setOpen] = useState(false);
  const [pointerMotion, setPointerMotion] = useState(false);

  useCloseOnDesktop(open, () => {
    setPointerMotion(false);
    setOpen(false);
  });

  return (
    <>
      <a className={styles.skipLink} href="#docs-content">
        Skip to content
      </a>
      <aside className={styles.desktop}>
        <SidebarNavigation apiReferences={apiReferences} path={currentPath} />
      </aside>
      <div className={styles.mobile}>
        <Dialog.Root
          open={open}
          onOpenChange={(next, { event }) => {
            setPointerMotion(isPointerActivation(event));
            setOpen(next);
          }}
        >
          <Dialog.Trigger className={styles.menuButton}>
            <Menu size={18} aria-hidden="true" />
            {title}
          </Dialog.Trigger>
          <Dialog.Portal>
            <Dialog.Backdrop className={styles.backdrop} data-pointer-motion={pointerMotion || undefined} />
            <Dialog.Popup className={styles.popup} data-pointer-motion={pointerMotion || undefined}>
              <div className={styles.popupHeader}>
                <Dialog.Title className={styles.popupTitle}>{title}</Dialog.Title>
                <Dialog.Close className={styles.closeButton} aria-label="Close navigation">
                  <X size={20} aria-hidden="true" />
                </Dialog.Close>
              </div>
              <SidebarNavigation
                apiReferences={apiReferences}
                path={currentPath}
                onNavigate={() => {
                  setPointerMotion(false);
                  setOpen(false);
                }}
              />
            </Dialog.Popup>
          </Dialog.Portal>
        </Dialog.Root>
        <DocsToc key={path} items={toc} compact />
      </div>
    </>
  );
}
