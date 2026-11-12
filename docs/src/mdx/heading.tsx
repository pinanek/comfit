import { Link as LinkIcon } from "lucide-react";
import { type ComponentProps, useId } from "react";
import styles from "./prose.module.css";

type HeadingTag = "h1" | "h2" | "h3" | "h4" | "h5" | "h6";

export function Heading({
  as: Tag,
  children,
  className,
  id,
  ...props
}: ComponentProps<"h2"> & { as: HeadingTag }) {
  const labelId = useId();

  return (
    <Tag
      {...props}
      id={id}
      data-pagefind-meta={Tag === "h1" ? "title" : undefined}
      aria-labelledby={props["aria-labelledby"] ?? (props["aria-label"] ? undefined : labelId)}
      className={[styles.heading, className].filter(Boolean).join(" ")}
    >
      {id && (
        <a className={styles.headingAnchor} href={`#${encodeURIComponent(id)}`} aria-label="Link to this heading">
          <LinkIcon size={16} aria-hidden="true" />
        </a>
      )}
      <span id={labelId}>{children}</span>
    </Tag>
  );
}
