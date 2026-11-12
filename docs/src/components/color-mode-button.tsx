"use client";

import { COLOR_MODE_KEY } from "#/lib/color-mode";
import { springConfig } from "#/lib/motion";
import { Button } from "@base-ui/react/button";
import { animated, useReducedMotion, useSpring } from "@react-spring/web";
import { useCallback, useEffect, useState } from "react";
import styles from "./color-mode-button.module.css";

const COLOR_MODE_EVENT = "comfit-color-mode-change";

// Matching path commands let the spring interpolate between the Lucide sun and moon.
const SUN_PATH =
  "M15.994 12.216C15.878 14.366 14.081 16.038 11.928 15.999C9.776 15.961 8.04 14.225 8.001 12.073C7.962 9.92 9.633 8.123 11.783 8.006C11.969 7.996 12.156 7.999 12.341 8.015C14.281 8.181 15.819 9.719 15.985 11.658C16.001 11.844 16.004 12.03 15.994 12.216Z";
const MOON_PATH =
  "M20.985 12.486C20.724 17.323 16.68 21.086 11.837 20.999C6.993 20.913 3.087 17.007 3 12.163C2.912 7.32 6.675 3.276 11.512 3.014C11.917 2.992 12.129 3.474 11.914 3.817C8.541 9.214 14.785 15.458 20.182 12.085C20.526 11.87 21.007 12.081 20.985 12.486Z";

function storedMode(): string | null {
  try {
    return localStorage.getItem(COLOR_MODE_KEY);
  } catch {
    return null;
  }
}

export function ColorModeButton() {
  const [mode, setMode] = useState<{ dark: boolean; animate: boolean } | null>(null);
  const reducedMotion = useReducedMotion();
  const { progress } = useSpring({
    progress: mode?.dark ? 1 : 0,
    immediate: !mode?.animate || reducedMotion !== false,
    config: springConfig,
  });

  const applyMode = useCallback((dark: boolean, animate = false) => {
    const root = document.documentElement;
    root.classList.toggle("dark", dark);
    root.style.colorScheme = dark ? "dark" : "light";
    setMode({ dark, animate });
  }, []);

  useEffect(() => {
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const sync = () => {
      const stored = storedMode();
      applyMode(stored === "dark" || (stored !== "light" && media.matches));
    };
    const onStorage = (event: StorageEvent) => {
      if (event.key === COLOR_MODE_KEY || event.key === null) sync();
    };

    const onModeChange = (event: Event) => applyMode((event as CustomEvent<boolean>).detail);

    sync();
    window.addEventListener(COLOR_MODE_EVENT, onModeChange);
    media.addEventListener("change", sync);
    window.addEventListener("storage", onStorage);

    return () => {
      window.removeEventListener(COLOR_MODE_EVENT, onModeChange);
      media.removeEventListener("change", sync);
      window.removeEventListener("storage", onStorage);
    };
  }, [applyMode]);

  const toggle = () => {
    const dark = !document.documentElement.classList.contains("dark");
    try {
      localStorage.setItem(COLOR_MODE_KEY, dark ? "dark" : "light");
    } catch {
      // The button still works when browser storage is unavailable.
    }
    window.dispatchEvent(new CustomEvent(COLOR_MODE_EVENT, { detail: dark }));
    applyMode(dark, true);
  };

  const label = mode ? mode.dark ? "Switch to light mode" : "Switch to dark mode" : "Toggle color mode";

  return (
    <Button
      type="button"
      className={styles.button}
      aria-label={label}
      aria-pressed={mode?.dark}
      title={label}
      onClick={toggle}
    >
      {mode && (
        <svg
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="currentColor"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
          aria-hidden="true"
        >
          <animated.path d={progress.to([0, 1], [SUN_PATH, MOON_PATH])} />
          <animated.g
            className={styles.rays}
            style={{ opacity: progress.to((value) => 1 - value), scale: progress.to((value) => 1 - value) }}
          >
            <path d="M12 4h.01M20 12h.01M12 20h.01M4 12h.01M17.657 6.343h.01M17.657 17.657h.01M6.343 17.657h.01M6.343 6.343h.01" />
          </animated.g>
        </svg>
      )}
    </Button>
  );
}
