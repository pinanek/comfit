import { Images, Scale } from "lucide-react";
import { GitHubIcon } from "./icons/github";
import { Link } from "./navigation-link";
import styles from "./site-footer.module.css";

export function SiteFooter() {
  return (
    <footer className={styles.footer} data-pagefind-ignore>
      <div className={styles.inner}>
        <p>© 2026 Ngo Duc Hoang Son</p>
        <nav className={styles.navigation} aria-label="Footer">
          <ul className={styles.links}>
            <li>
              <Link to="/logos" aria-label="Logos" title="Logos">
                <Images size={16} aria-hidden="true" />
              </Link>
            </li>
            <li>
              <a href="/LICENSE" aria-label="MIT License" title="MIT License">
                <Scale size={16} aria-hidden="true" />
              </a>
            </li>
            <li>
              <a href="https://github.com/pinanek/comfit" aria-label="GitHub" title="GitHub">
                <GitHubIcon width={16} height={16} />
              </a>
            </li>
          </ul>
        </nav>
      </div>
    </footer>
  );
}
