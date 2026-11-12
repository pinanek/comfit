import { Link } from "#/components/navigation-link";
import { ArrowLeft, ArrowRight } from "lucide-react";
import styles from "./footer.module.css";
import { sections } from "./navigation";

const pages = sections.flatMap((section) => section.items).filter((item) => "href" in item);

export function DocsFooter({ path }: { path: string }) {
  const index = pages.findIndex((page) => page.href === path);
  if (index === -1) return null;

  const previous = pages[index - 1];
  const next = pages[index + 1];

  return (
    <footer className={styles.footer} data-pagefind-ignore>
      <nav className={styles.navigation} aria-label="Page navigation">
        {previous && (
          <Link className={styles.link} to={previous.href} rel="prev">
            <ArrowLeft size={16} aria-hidden="true" />
            <span className={styles.text}>
              <span className={styles.direction}>Previous</span>
              <span className={styles.title}>{previous.label}</span>
            </span>
          </Link>
        )}
        {next && (
          <Link className={`${styles.link} ${styles.next}`} to={next.href} rel="next">
            <span className={styles.text}>
              <span className={styles.direction}>Next</span>
              <span className={styles.title}>{next.label}</span>
            </span>
            <ArrowRight size={16} aria-hidden="true" />
          </Link>
        )}
      </nav>
    </footer>
  );
}
