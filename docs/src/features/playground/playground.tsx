"use client";

import { SelectField } from "#/components/select-field";
import type { PlaygroundTheme } from "#/features/playground/themes";
import { feedbackSpringConfig } from "#/lib/motion";
import codeStyles from "#/mdx/code-block.module.css";
import { CopyButton } from "#/mdx/copy-button";
import { Button } from "@base-ui/react/button";
import { Tabs } from "@base-ui/react/tabs";
import { animated, useReducedMotion, useSpring } from "@react-spring/web";
import { Check, Rainbow, RotateCcw } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import { Editor } from "./editor";
import { type LanguageId, languages } from "./languages";
import styles from "./playground.module.css";
import { usePlaygroundEngine } from "./use-playground-engine";

const outputFormats = [
  { value: "html", label: "HTML", description: "Generated HTML" },
  { value: "tokens", label: "Tokens", description: "Token JSON" },
] as const;

export function Playground({ themes }: { themes: PlaygroundTheme[] }) {
  const [language, setLanguage] = useState<LanguageId>("rust");
  const [sources, setSources] = useState(() =>
    Object.fromEntries(languages.map(({ id, sample }) => [id, sample])) as Record<LanguageId, string>
  );
  const [themeId, setThemeId] = useState("site");
  const [rainbow, setRainbow] = useState(true);
  const [highlightedLines, setHighlightedLines] = useState<number[]>([]);
  const [view, setView] = useState("html");
  const [loaded, setLoaded] = useState(false);
  const [pointerLoad, setPointerLoad] = useState(false);
  const loadTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const reducedMotion = useReducedMotion();
  const { loadProgress } = useSpring({
    loadProgress: loaded ? 1 : 0,
    immediate: !pointerLoad || reducedMotion !== false,
    config: feedbackSpringConfig,
  });
  const source = sources[language];
  const currentLanguage = languages.find(({ id }) => id === language)!;
  const theme = themes.find(({ id }) => id === themeId)?.theme;
  const { highlighting, output, error, pending, retry } = usePlaygroundEngine({
    source,
    language,
    theme,
    rainbow,
    highlightedLines,
  });

  const updateSource = useCallback((value: string) => {
    setSources((previous) => previous[language] === value ? previous : { ...previous, [language]: value });
  }, [language]);

  const toggleLine = useCallback((line: number) => {
    setHighlightedLines((previous) => {
      const selected = new Set(previous);
      if (selected.has(line)) selected.delete(line);
      else selected.add(line);
      return [...selected].sort((left, right) => left - right);
    });
  }, []);

  useEffect(() => () => clearTimeout(loadTimer.current), []);

  const copyText = view === "tokens" ? output?.tokens : output?.html;
  const loadLabel = loaded ? "Example loaded" : "Load example";

  return (
    <div className={styles.playground}>
      <div className={styles.intro}>
        <h1>Playground</h1>
        <p>Edit code and inspect Comfit’s HTML and tokens. Everything runs in your browser.</p>
      </div>

      <div className={styles.workspace}>
        <section className={styles.pane} aria-label="Source code">
          <div className={styles.paneHeader}>
            <SelectField
              compact
              label="Theme"
              items={[
                { value: "site", label: "Site theme" },
                ...themes.map(({ id, theme }) => ({ value: id, label: theme.name ?? id })),
              ]}
              value={themeId}
              onValueChange={setThemeId}
            />
            <Button
              type="button"
              className={styles.rainbowToggle}
              aria-label="Rainbow brackets"
              aria-pressed={rainbow}
              title="Rainbow brackets"
              onClick={() => setRainbow((previous) => !previous)}
            >
              <Rainbow size={14} aria-hidden="true" />
              <span>Rainbow</span>
            </Button>
            <SelectField
              compact
              label="Language"
              items={languages.map(({ id, name }) => ({ value: id, label: name }))}
              value={language}
              onValueChange={setLanguage}
            />
          </div>
          <div className={codeStyles.body}>
            <Button
              type="button"
              className={codeStyles.copyButton}
              aria-label={loadLabel}
              title={loadLabel}
              data-copy-state={loaded ? "copied" : "idle"}
              onClick={(event) => {
                clearTimeout(loadTimer.current);
                updateSource(currentLanguage.sample);
                setPointerLoad(event.detail > 0);
                setLoaded(true);
                loadTimer.current = setTimeout(() => setLoaded(false), 2000);
              }}
            >
              <animated.span
                className={codeStyles.copyIcon}
                aria-hidden="true"
                style={{ opacity: loadProgress.to((value) => 1 - value), scale: loadProgress.to([0, 1], [1, 0.65]) }}
              >
                <RotateCcw size={16} />
              </animated.span>
              <animated.span
                className={codeStyles.copyIcon}
                aria-hidden="true"
                style={{ opacity: loadProgress, scale: loadProgress.to([0, 1], [0.5, 1]) }}
              >
                <Check size={16} />
              </animated.span>
            </Button>
            <output className={codeStyles.status}>{loaded ? "Example loaded" : ""}</output>
            <Editor
              key={language}
              value={source}
              onChange={updateSource}
              onToggleLine={toggleLine}
              highlighting={!error && highlighting?.language === language ? highlighting : null}
            />
          </div>
        </section>

        <Tabs.Root className={styles.pane} value={view} onValueChange={setView}>
          <div className={styles.outputHeader}>
            <Tabs.List className={styles.tabs} aria-label="Output format">
              {outputFormats.map(({ value, label }) => (
                <Tabs.Tab key={value} className={styles.tab} value={value}>{label}</Tabs.Tab>
              ))}
            </Tabs.List>
          </div>
          <div className={styles.outputBody}>
            {!pending && copyText !== undefined && <CopyButton text={copyText} />}
            <div className={styles.output} aria-busy={pending !== false}>
              {error
                ? (
                  <div className={styles.message} role="alert">
                    <p>{error}</p>
                    <Button type="button" className={styles.action} onClick={retry}>
                      Try again
                    </Button>
                  </div>
                )
                : output
                ? (
                  outputFormats.map(({ value, description }) => (
                    <Tabs.Panel key={value} value={value} className={styles.raw}>
                      <pre className={styles.rawCode} tabIndex={0} aria-label={description}>
                        <code>{output[value]}</code>
                      </pre>
                    </Tabs.Panel>
                  ))
                )
                : <p className={styles.message}>Loading the highlighting engine…</p>}
            </div>
          </div>
        </Tabs.Root>
      </div>
    </div>
  );
}
