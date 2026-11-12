import type { ComponentProps } from "react";
import styles from "./table.module.css";

export function Table({ className, ...props }: ComponentProps<"table">) {
  return (
    <section className={styles.container} tabIndex={0} aria-label={props["aria-label"] ?? "Table"}>
      <table {...props} className={[styles.table, className].filter(Boolean).join(" ")} />
    </section>
  );
}
