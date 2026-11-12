"use client";

import { feedbackSpringConfig } from "#/lib/motion";
import { Button } from "@base-ui/react/button";
import { animated, useReducedMotion, useSpring } from "@react-spring/web";
import { Check, Copy, TriangleAlert } from "lucide-react";
import { type MouseEvent, useEffect, useRef, useState } from "react";
import styles from "./code-block.module.css";

export function CopyButton({ text }: { text: string }) {
  const [state, setState] = useState<"idle" | "copying" | "copied" | "error">("idle");
  const [pointerCopy, setPointerCopy] = useState(false);
  const timer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const mounted = useRef(true);
  const reducedMotion = useReducedMotion();
  const { progress } = useSpring({
    progress: state === "copied" ? 1 : 0,
    immediate: !pointerCopy || reducedMotion !== false,
    config: feedbackSpringConfig,
  });

  useEffect(() => {
    mounted.current = true;
    return () => {
      mounted.current = false;
      clearTimeout(timer.current);
    };
  }, []);

  const copy = async (event: MouseEvent<HTMLButtonElement>) => {
    clearTimeout(timer.current);
    setPointerCopy(event.detail > 0);
    setState("copying");

    try {
      if (navigator.clipboard && globalThis.isSecureContext) {
        await navigator.clipboard.writeText(text);
      } else {
        const previousFocus = document.activeElement;
        const textarea = document.createElement("textarea");
        textarea.value = text;
        textarea.readOnly = true;
        textarea.tabIndex = -1;
        textarea.setAttribute("aria-hidden", "true");
        Object.assign(textarea.style, { position: "fixed", inset: "0", opacity: "0", pointerEvents: "none" });
        document.body.append(textarea);

        try {
          textarea.select();
          if (!document.execCommand("copy")) throw new Error("Unable to copy code");
        } finally {
          textarea.remove();
          if (previousFocus instanceof HTMLElement) previousFocus.focus({ preventScroll: true });
        }
      }
      if (!mounted.current) return;
      setState("copied");
    } catch {
      if (!mounted.current) return;
      setState("error");
    }

    timer.current = setTimeout(() => setState("idle"), 2000);
  };

  const label = state === "copied" ? "Copied" : state === "error" ? "Copy failed. Try again" : "Copy code";

  return (
    <>
      <Button
        type="button"
        className={styles.copyButton}
        onClick={copy}
        disabled={state === "copying"}
        aria-label={label}
        title={label}
        data-copy-state={state}
      >
        <animated.span
          className={styles.copyIcon}
          aria-hidden="true"
          style={{
            opacity: state === "error" ? 0 : progress.to((value) => 1 - value),
            scale: progress.to([0, 1], [1, 0.65]),
          }}
        >
          <Copy size={16} />
        </animated.span>
        <animated.span
          className={styles.copyIcon}
          aria-hidden="true"
          style={{ opacity: state === "error" ? 0 : progress, scale: progress.to([0, 1], [0.5, 1]) }}
        >
          <Check size={16} />
        </animated.span>
        {state === "error" && (
          <span className={styles.copyIcon} aria-hidden="true">
            <TriangleAlert size={16} />
          </span>
        )}
      </Button>
      <output className={styles.status}>
        {state === "copied" ? "Code copied to clipboard" : state === "error" ? "Could not copy code. Try again." : ""}
      </output>
    </>
  );
}
