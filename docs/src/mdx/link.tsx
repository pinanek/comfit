import { Link } from "#/components/navigation-link";
import { ArrowUpRight } from "lucide-react";
import type { ComponentProps } from "react";
import styles from "./prose.module.css";

export function MarkdownLink({ children, className, href, ...props }: ComponentProps<"a">) {
  if (!href) return <a {...props} className={className}>{children}</a>;

  const external = /^(?:https?:)?\/\//i.test(href);
  const internal = !/^(?:[a-z][a-z\d+.-]*:|\/\/)/i.test(href)
    && (props.download === undefined || props.download === null || props.download === false)
    && (!props.target || props.target.toLowerCase() === "_self");
  const linkClass = [styles.link, className].filter(Boolean).join(" ");
  const content = (
    <>
      <span className={styles.linkLabel}>{children}</span>
      {external && <ArrowUpRight className={styles.externalIcon} size={12} aria-hidden="true" />}
    </>
  );

  return internal
    ? <Link {...props} to={href} className={linkClass}>{content}</Link>
    : (
      <a
        {...props}
        href={href}
        className={linkClass}
        target={external ? "_blank" : props.target}
        rel={external ? [props.rel, "noopener noreferrer"].filter(Boolean).join(" ") : props.rel}
      >
        {content}
      </a>
    );
}
