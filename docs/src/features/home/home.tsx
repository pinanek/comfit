import { Metadata } from "#/components/metadata";
import { Link } from "#/components/navigation-link";
import { HomeStory } from "#/features/home/home-story";
import HomeResources from "#/features/home/resources.md";
import { playgroundThemes } from "#/features/playground/themes";
import { CodeBlock } from "#/mdx/code-block";
import { CodeTab, CodeTabs } from "#/mdx/code-tabs";
import { site } from "#/site";
import styles from "./home.module.css";

export function Home() {
  return (
    <div className={styles.home}>
      <Metadata />
      <header className={styles.intro}>
        <h1>Code has structure. Make it visible.</h1>
        <p>{site.description}</p>
        <p>Turn source code into highlighted tokens, HTML, or terminal output.</p>
        <div className={styles.actions}>
          <Link to="/playground">Try the playground</Link>
          <Link to="/docs">Read the docs</Link>
        </div>
      </header>

      <HomeStory themes={playgroundThemes.filter(({ id }) => id === "catppina-light" || id === "catppina-dark")} />

      <div className={styles.resources}>
        <HomeResources components={{ CodeTab, CodeTabs, Link, pre: CodeBlock }} />
      </div>

      <section className={styles.closing} aria-labelledby="home-try-code">
        <h2 id="home-try-code">Try your own code.</h2>
        <p>Pick a language, paste a snippet, and see what Comfit highlights.</p>
        <Link to="/playground">Open the playground</Link>
        <nav className={styles.references} aria-label="API references">
          <Link to="/api-references/rust">Rust</Link>
          <Link to="/api-references/javascript/node">JavaScript</Link>
          <Link to="/api-references/python">Python</Link>
        </nav>
      </section>
    </div>
  );
}
