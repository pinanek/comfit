import type { ReactNode } from "react";
import styles from "./alert.module.css";

const labels = {
  note: "Note",
  tip: "Tip",
  important: "Important",
  warning: "Warning",
  caution: "Caution",
};

export function Alert({ kind, children }: { kind: keyof typeof labels; children: ReactNode }) {
  return (
    <aside className={styles.alert} data-kind={kind} aria-label={labels[kind]}>
      <div className={styles.header}>{labels[kind]}</div>
      <div className={styles.content}>{children}</div>
    </aside>
  );
}
