import type { ComponentProps } from "react";
import styles from "./prose.module.css";

export function Image({ className, alt, ...props }: ComponentProps<"img">) {
  return (
    <img
      alt={alt ?? ""}
      loading="lazy"
      decoding="async"
      {...props}
      className={[styles.image, className].filter(Boolean).join(" ")}
    />
  );
}
