"use client";

import { SelectField } from "#/components/select-field";
import styles from "#/features/home/home.module.css";
import type { PlaygroundTheme } from "#/features/playground/themes";
import { springConfig } from "#/lib/motion";
import { animated, useSpring } from "@react-spring/web";
import { type ReactNode, useEffect, useRef, useState } from "react";
import { type Output, SampleCode, type Scene } from "./sample-code";

const chapters = [
  { scene: "source", title: "Start with the source." },
  { scene: "injection", title: "Find the language inside the language." },
  { scene: "roles", title: "Give each token its role." },
  { scene: "theme", title: "Choose how the code reads." },
  { scene: "output", title: "Put it where your code appears." },
] as const;

const outputItems: { value: Output; label: string }[] = [
  { value: "html", label: "HTML" },
  { value: "ansi", label: "ANSI" },
  { value: "tokens", label: "Tokens" },
];
const desktopStory = "(width >= 62rem) and (height >= 37rem) and (prefers-reduced-motion: no-preference)";
const opening = { chapter: -1, beat: 0 };

const descriptions: Record<Exclude<Scene, "opening">, ReactNode> = {
  source: <p>A heading. A code block. A few lines of Rust. Comfit starts with the text you already have.</p>,
  injection: (
    <>
      <p>A Rust code block is still Rust, even inside Markdown.</p>
      <p>Comfit follows embedded languages so each part gets its own highlighting.</p>
    </>
  ),
  roles: (
    <>
      <p>Keywords, strings, functions, and local references have different jobs.</p>
      <p>
        Tree-sitter queries identify what to highlight. Comfit resolves overlapping captures and carries local-binding
        styles through their references.
      </p>
      <p>
        Here, <code>let</code> is a keyword, <code>"Comfit"</code> is a string, and <code>name</code>{" "}
        is a local binding.
      </p>
    </>
  ),
  theme: (
    <>
      <p>Keep the highlighting. Change the presentation.</p>
      <p>Apply a theme, color nested brackets, or emphasize the lines you want readers to notice.</p>
    </>
  ),
  output: (
    <>
      <p>Render HTML for a website, ANSI for a terminal, or tokens for your own interface.</p>
      <p>The shared highlighting engine is available through Rust, JavaScript, and Python integrations.</p>
    </>
  ),
};

function ChapterMarker({ active }: { active: boolean }) {
  const motion = useSpring({ opacity: active ? 1 : 0, scaleY: active ? 1 : 0.95, config: springConfig });
  return <animated.span aria-hidden="true" className={styles.chapterMarker} style={motion} />;
}

export function HomeStory({ themes }: { themes: PlaygroundTheme[] }) {
  const [themeChoice, setThemeChoice] = useState<string>();
  const [outputChoice, setOutputChoice] = useState<Output>();
  const [position, setPosition] = useState(opening);
  const [siteDark, setSiteDark] = useState(false);
  const sections = useRef<(HTMLElement | null)[]>([]);
  const frame = useRef<number | undefined>(undefined);

  useEffect(() => {
    const media = window.matchMedia(desktopStory);
    const root = document.documentElement;
    const syncSiteTheme = () => setSiteDark(root.classList.contains("dark"));
    const observer = new MutationObserver(syncSiteTheme);
    observer.observe(root, { attributeFilter: ["class"] });
    syncSiteTheme();

    const measure = () => {
      frame.current = undefined;
      if (!media.matches) {
        setPosition(opening);
        return;
      }

      const anchor = window.innerHeight * 0.42;
      let next = opening;
      for (const [chapter, section] of sections.current.entries()) {
        const bounds = section?.getBoundingClientRect();
        if (!bounds || bounds.top > anchor) break;
        const beat = Math.min(2, Math.floor(3 * (anchor - bounds.top) / bounds.height));
        next = { chapter, beat };
      }

      setPosition((previous) => previous.chapter === next.chapter && previous.beat === next.beat ? previous : next);
    };

    const schedule = () => {
      if (frame.current === undefined) frame.current = requestAnimationFrame(measure);
    };

    schedule();
    window.addEventListener("scroll", schedule, { passive: true });
    window.addEventListener("resize", schedule);
    media.addEventListener("change", schedule);

    return () => {
      if (frame.current !== undefined) {
        cancelAnimationFrame(frame.current);
        frame.current = undefined;
      }
      observer.disconnect();
      window.removeEventListener("scroll", schedule);
      window.removeEventListener("resize", schedule);
      media.removeEventListener("change", schedule);
    };
  }, []);

  const scene = chapters[position.chapter]?.scene ?? "opening";
  const selectedTheme = (id: string) => themes.find((theme) => theme.id === id)?.theme;
  const themeId = themeChoice ?? (siteDark ? "catppina-dark" : "catppina-light");
  const output = outputChoice ?? (scene === "output" ? outputItems[position.beat]!.value : "html");

  const showPreview = (chapter: number) => {
    if (window.matchMedia(desktopStory).matches) {
      setPosition((previous) => previous.chapter === chapter ? previous : { chapter, beat: 0 });
    }
  };

  const panelTheme = (panelScene: Scene) => {
    if (panelScene === "theme") return selectedTheme(themeId);
    if (panelScene === "output") {
      return selectedTheme(output === "ansi" ? "catppina-dark" : themeId);
    }
    return undefined;
  };

  return (
    <section className={styles.story} aria-label="From source to screen">
      <div className={styles.stage}>
        <SampleCode
          scene={scene}
          role={position.beat}
          rustFocus={scene === "injection" && position.beat > 0}
          emphasize={scene === "theme" && position.beat === 2}
          output={output}
          theme={panelTheme(scene)}
          animatedInk
        />
      </div>

      <div className={styles.chapters}>
        {chapters.map(({ scene, title }, index) => (
          <article
            className={styles.chapter}
            key={scene}
            aria-current={index === position.chapter ? "step" : undefined}
            ref={(section) => {
              sections.current[index] = section;
            }}
          >
            <div className={styles.chapterContent}>
              <ChapterMarker active={index === position.chapter} />
              <h2>{title}</h2>
              {descriptions[scene]}

              {scene === "theme" && (
                <div className={styles.control}>
                  <SelectField
                    label="Theme"
                    items={themes.map(({ id, theme }) => ({ value: id, label: theme.name ?? id }))}
                    value={themeId}
                    onValueChange={(value) => {
                      setThemeChoice(value);
                      showPreview(index);
                    }}
                  />
                </div>
              )}

              {scene === "output" && (
                <div className={styles.control}>
                  <SelectField
                    label="Output format"
                    items={outputItems}
                    value={output}
                    onValueChange={(value) => {
                      setOutputChoice(value);
                      showPreview(index);
                    }}
                  />
                </div>
              )}
            </div>

            <div className={styles.inlineSample}>
              <SampleCode
                scene={scene}
                rustFocus={scene === "injection"}
                emphasize={scene === "theme"}
                output={output}
                theme={panelTheme(scene)}
              />
            </div>
          </article>
        ))}
      </div>
    </section>
  );
}
