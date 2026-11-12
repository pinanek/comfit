import styles from "#/features/documentation/layout.module.css";
import type { ReactNode } from "react";

export default function DocsLayout({ children }: { children: ReactNode }) {
  return (
    <div className={styles.layout}>
      {children}
    </div>
  );
}

export const getConfig = async () => {
  return {
    render: "static",
  } as const;
};
