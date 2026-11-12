import { springConfig } from "#/lib/motion";
import { animated, to, useReducedMotion, useSpring } from "@react-spring/web";
import { type RefObject, useEffect, useId, useRef } from "react";
import styles from "./sample-code.module.css";

export function TokenCallout({ container, label, role, animate }: {
  container: RefObject<HTMLDivElement | null>;
  label: string;
  role: number;
  animate: boolean;
}) {
  const labelRef = useRef<HTMLDivElement>(null);
  const positioned = useRef(false);
  const markerId = useId();
  const reducedMotion = useReducedMotion();
  const [motion, api] = useSpring(() => ({
    x: 0,
    y: 0,
    width: 0,
    height: 0,
    labelX: 0,
    labelY: 0,
    labelWidth: 0,
    labelHeight: 0,
    railX: 0,
    routeY: 0,
    opacity: 0,
    config: springConfig,
  }));

  useEffect(() => {
    const element = container.current;
    const labelElement = labelRef.current;
    const token = element?.querySelector<HTMLElement>("[data-focused]");
    const pre = element?.querySelector("pre");
    if (!element || !labelElement || !token || !pre) return;

    const lifecycle = new AbortController();
    const scheduled = { frame: 0 };
    const measure = () => {
      scheduled.frame = 0;
      const bounds = element.getBoundingClientRect();
      const target = token.getBoundingClientRect();
      const labelBounds = labelElement.getBoundingClientRect();
      const viewport = pre.getBoundingClientRect();
      if (!bounds.width || !labelBounds.width) {
        api.set({ opacity: 0 });
        return;
      }

      const lineHeight = Number.parseFloat(getComputedStyle(token.parentElement!).lineHeight);
      const narrow = window.matchMedia("(width < 35rem)").matches;
      const sourceRight = Math.max(
        ...Array.from(
          element.querySelectorAll("[data-embedded] > span"),
          (span) => span.getBoundingClientRect().right - bounds.left,
        ),
      );
      const labelX = narrow
        ? 24
        : Math.min(bounds.width - labelBounds.width - 24, Math.max(bounds.width * 0.58, sourceRight + 32));
      const labelY = narrow
        ? bounds.height - labelBounds.height - 20
        : Math.max(
          44,
          Math.min(
            bounds.height - labelBounds.height - 16,
            target.top - bounds.top + target.height / 2 + [-2, 0, 3.25][role]! * lineHeight - labelBounds.height / 2,
          ),
        );
      const visible = target.left >= viewport.left + 4 && target.right <= viewport.right - 4;

      void api.start({
        x: target.left - bounds.left - 4,
        y: target.top - bounds.top - 4,
        width: target.width + 8,
        height: target.height + 8,
        labelX,
        labelY,
        labelWidth: labelBounds.width,
        labelHeight: labelBounds.height,
        railX: narrow ? 12 : labelX + labelBounds.width + 12,
        // Route through the blank line below the binding, clear of the source and line number.
        routeY: target.bottom - bounds.top + lineHeight * (narrow ? 1 : 0.65),
        opacity: visible ? 1 : 0,
        immediate: !animate || reducedMotion !== false || !positioned.current,
      });
      positioned.current = true;
    };
    const schedule = () => {
      if (!lifecycle.signal.aborted && !scheduled.frame) scheduled.frame = requestAnimationFrame(measure);
    };
    const observer = new ResizeObserver(schedule);
    observer.observe(element);
    observer.observe(labelElement);
    pre.addEventListener("scroll", schedule, { passive: true });
    void document.fonts.ready.then(schedule);
    schedule();

    return () => {
      lifecycle.abort();
      observer.disconnect();
      pre.removeEventListener("scroll", schedule);
      cancelAnimationFrame(scheduled.frame);
    };
  }, [animate, api, container, reducedMotion, role]);

  const connector = to(
    [
      motion.x,
      motion.y,
      motion.width,
      motion.height,
      motion.labelX,
      motion.labelY,
      motion.labelWidth,
      motion.labelHeight,
      motion.railX,
      motion.routeY,
    ],
    (x, y, width, height, labelX, labelY, labelWidth, labelHeight, railX, routeY) => {
      const startX = railX > labelX ? labelX + labelWidth + 6 : labelX - 6;
      return `M ${startX} ${labelY + labelHeight / 2} H ${railX} V ${routeY} H ${x + width / 2} V ${y + height + 3}`;
    },
  );

  return (
    <>
      <animated.svg className={styles.roleOverlay} aria-hidden="true" style={{ opacity: motion.opacity }}>
        <defs>
          <marker
            id={markerId}
            viewBox="0 0 10 10"
            refX="9"
            refY="5"
            markerWidth="8"
            markerHeight="8"
            markerUnits="userSpaceOnUse"
            orient="auto"
          >
            <path d="M 1 1 L 9 5 L 1 9 Z" fill="currentColor" />
          </marker>
        </defs>
        <animated.rect
          className={styles.focusBox}
          x={motion.x}
          y={motion.y}
          width={motion.width}
          height={motion.height}
          rx="3"
        />
        <animated.path className={styles.roleConnector} d={connector} markerEnd={`url(#${markerId})`} />
      </animated.svg>
      <animated.div
        ref={labelRef}
        className={styles.roleLabel}
        aria-hidden="true"
        style={{
          opacity: motion.opacity,
          transform: to([motion.labelX, motion.labelY], (x, y) => `translate3d(${x}px, ${y}px, 0)`),
        }}
      >
        {label}
      </animated.div>
    </>
  );
}
