"use client";

import { Tabs } from "@base-ui/react/tabs";
import { type ReactNode, useState } from "react";
import styles from "./code-tabs.module.css";
import { languageName } from "./language-name";

export interface CodeTabDefinition {
  label: string;
  language?: string;
  title?: string;
}

interface CodeTabsProps {
  tabs: CodeTabDefinition[];
  label?: string;
  children: ReactNode;
}

export function CodeTabs({ tabs, label = "Code examples", children }: CodeTabsProps) {
  const [value, setValue] = useState(tabs[0]!.label);
  const selected = tabs.find((tab) => tab.label === value) ?? tabs[0]!;
  const detail = selected.title || languageName(selected.language);

  return (
    <Tabs.Root className={styles.root} value={selected.label} onValueChange={setValue}>
      <div className={styles.header}>
        <Tabs.List className={styles.list} aria-label={label} activateOnFocus>
          {tabs.map((tab) => (
            <Tabs.Tab className={styles.tab} key={tab.label} value={tab.label}>
              {tab.label}
            </Tabs.Tab>
          ))}
        </Tabs.List>
        <span className={styles.detail} title={detail}>{detail}</span>
      </div>
      {children}
    </Tabs.Root>
  );
}

export function CodeTab({ label, children }: { label: string; children: ReactNode }) {
  return (
    <Tabs.Panel className={styles.panel} value={label} keepMounted>
      {children}
    </Tabs.Panel>
  );
}
