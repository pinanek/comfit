import type { ComponentProps } from "react";
import styles from "./code-block.module.css";
import { CopyButton } from "./copy-button";
import { languageName } from "./language-name";

interface CodeBlockProps extends ComponentProps<"pre"> {
  "data-code"?: string;
  "data-language"?: string;
  "data-title"?: string;
}

export function CodeBlock({
  children,
  className,
  "data-code": source = "",
  "data-language": language = "text",
  "data-title": title,
  ...props
}: CodeBlockProps) {
  const label = languageName(language);

  return (
    <figure className={styles.block} aria-label={title ?? `${label} code`}>
      <figcaption className={styles.header}>
        <span className={styles.title} title={title}>{title}</span>
        <span className={styles.language}>{label}</span>
      </figcaption>
      <div className={styles.body}>
        <CopyButton text={source} />
        <pre {...props} className={[styles.pre, className].filter(Boolean).join(" ")} tabIndex={0}>
          {children}
        </pre>
      </div>
    </figure>
  );
}
