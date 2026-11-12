"use client";

import type { TocItem } from "#/features/documentation/types";
import { useCloseOnDesktop } from "#/hooks/use-close-on-desktop";
import { Collapsible } from "@base-ui/react/collapsible";
import { Popover } from "@base-ui/react/popover";
import { ChevronDown } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import styles from "./toc.module.css";

interface Geometry {
  path: string;
  height: number;
  length: number;
  segments: { id: string; start: number; end: number }[];
}

function TocList({ items, active, onNavigate }: { items: TocItem[]; active: string[]; onNavigate?: () => void }) {
  const listRef = useRef<HTMLOListElement>(null);
  const [geometry, setGeometry] = useState<Geometry | null>(null);

  useEffect(() => {
    const list = listRef.current;
    if (!list) return;

    const path = document.createElementNS("http://www.w3.org/2000/svg", "path");

    const scheduled = { frame: 0 };
    const lifecycle = new AbortController();

    const build = () => {
      scheduled.frame = 0;
      const bounds = list.getBoundingClientRect();
      if (bounds.width === 0) return;

      const links = Array.from(list.querySelectorAll("a"));
      const positions = links.map((link) => {
        const rect = link.getBoundingClientRect();
        const style = getComputedStyle(link);
        return {
          x: rect.left - bounds.left - 8,
          top: rect.top - bounds.top + parseFloat(style.paddingBlockStart),
          bottom: rect.bottom - bounds.top - parseFloat(style.paddingBlockEnd),
        };
      });
      const commands: (string | number)[] = [];
      const segments: Geometry["segments"] = [];
      const measure = () => {
        path.setAttribute("d", commands.join(" "));
        return path.getTotalLength();
      };

      for (const [index, position] of positions.entries()) {
        const previous = positions[index - 1];
        if (!previous) {
          commands.push("M", position.x, position.top);
        } else if (Math.abs(position.x - previous.x) < 0.5) {
          commands.push("V", position.top);
        } else {
          const middle = (previous.bottom + position.top) / 2;
          const direction = Math.sign(position.x - previous.x);
          const radius = Math.min(
            6,
            Math.abs(position.x - previous.x) / 2,
            Math.max((position.top - previous.bottom) / 2, 0),
          );
          commands.push(
            "V",
            middle - radius,
            "Q",
            previous.x,
            middle,
            previous.x + direction * radius,
            middle,
            "H",
            position.x - direction * radius,
            "Q",
            position.x,
            middle,
            position.x,
            middle + radius,
            "V",
            position.top,
          );
        }

        const start = measure();
        commands.push("V", position.bottom);
        segments.push({ id: items[index]!.id, start, end: measure() });
      }

      setGeometry({
        path: commands.join(" "),
        height: list.scrollHeight,
        length: segments.at(-1)?.end ?? 0,
        segments,
      });
    };
    const requestBuild = () => {
      if (!lifecycle.signal.aborted && !scheduled.frame) scheduled.frame = requestAnimationFrame(build);
    };
    const observer = new ResizeObserver(requestBuild);
    observer.observe(list);
    requestBuild();
    void document.fonts.ready.then(requestBuild);

    return () => {
      lifecycle.abort();
      observer.disconnect();
      cancelAnimationFrame(scheduled.frame);
    };
  }, [items]);

  const selected = geometry?.segments.filter(({ id }) => active.includes(id)) ?? [];
  const start = selected[0]?.start ?? 0;
  const end = selected.at(-1)?.end ?? start;

  return (
    <div className={styles.body}>
      <svg className={styles.indicator} height={geometry?.height ?? 0} aria-hidden="true">
        {geometry && (
          <path
            key={geometry.path}
            className={styles.path}
            d={geometry.path}
            strokeDasharray={`0 ${start} ${end - start} ${geometry.length + 1}`}
            opacity={end > start ? 1 : 0}
          />
        )}
      </svg>
      <ol className={styles.list} ref={listRef}>
        {items.map((item) => (
          <li key={item.id} data-depth={item.depth} data-active={active.includes(item.id) || undefined}>
            <a
              href={`#${encodeURIComponent(item.id)}`}
              aria-current={active[0] === item.id ? "location" : undefined}
              onClick={onNavigate}
            >
              {item.title}
            </a>
          </li>
        ))}
      </ol>
    </div>
  );
}

export function DocsToc({ items, compact = false }: { items: TocItem[]; compact?: boolean }) {
  const [active, setActive] = useState<string[]>(items[0] ? [items[0].id] : []);
  const [open, setOpen] = useState(false);

  useCloseOnDesktop(compact && open, () => setOpen(false));

  useEffect(() => {
    const headings = items.flatMap((item) => {
      const heading = document.getElementById(item.id);
      return heading ? [{ id: item.id, heading }] : [];
    });
    if (!headings.length) return;

    const visible = new Set<Element>();
    const scheduled = { frame: 0 };
    const lifecycle = new AbortController();

    const update = () => {
      scheduled.frame = 0;
      let selected = headings.filter(({ heading }) => visible.has(heading)).map(({ id }) => id);

      if (!selected.length) {
        const offset = Math.min(window.innerHeight * 0.25, 180);
        let current = headings[0]!;
        for (const entry of headings) {
          if (entry.heading.getBoundingClientRect().top > offset) break;
          current = entry;
        }
        selected = [current.id];
      }

      setActive((previous) =>
        previous.length === selected.length && previous.every((id, index) => id === selected[index])
          ? previous
          : selected
      );
    };
    const requestUpdate = () => {
      if (!lifecycle.signal.aborted && !scheduled.frame) scheduled.frame = requestAnimationFrame(update);
    };
    const observer = new IntersectionObserver((changes) => {
      for (const change of changes) {
        if (change.isIntersecting) visible.add(change.target);
        else visible.delete(change.target);
      }
      requestUpdate();
    });
    for (const { heading } of headings) observer.observe(heading);
    window.addEventListener("scroll", requestUpdate, { passive: true, signal: lifecycle.signal });
    window.addEventListener("resize", requestUpdate, { signal: lifecycle.signal });
    requestUpdate();

    return () => {
      lifecycle.abort();
      observer.disconnect();
      cancelAnimationFrame(scheduled.frame);
    };
  }, [items]);

  if (!items.length) return null;

  if (compact) {
    return (
      <nav aria-label="On this page">
        <Popover.Root open={open} onOpenChange={setOpen}>
          <Popover.Trigger className={styles.compactTrigger}>
            On this page
            <ChevronDown className={styles.chevron} size={16} aria-hidden="true" />
          </Popover.Trigger>
          <Popover.Portal>
            <Popover.Positioner className={styles.positioner} align="end" sideOffset={8} collisionPadding={12}>
              <Popover.Popup className={styles.popup}>
                <Popover.Title className={styles.title}>On this page</Popover.Title>
                <TocList items={items} active={active} onNavigate={() => setOpen(false)} />
              </Popover.Popup>
            </Popover.Positioner>
          </Popover.Portal>
        </Popover.Root>
      </nav>
    );
  }

  return (
    <nav className={styles.root} aria-label="On this page">
      <div className={styles.desktop}>
        <h2 className={styles.title}>On this page</h2>
        <TocList items={items} active={active} />
      </div>
      <Collapsible.Root className={styles.mobile}>
        <Collapsible.Trigger className={styles.trigger}>
          On this page
          <ChevronDown className={styles.chevron} size={16} aria-hidden="true" />
        </Collapsible.Trigger>
        <Collapsible.Panel>
          <TocList items={items} active={active} />
        </Collapsible.Panel>
      </Collapsible.Root>
    </nav>
  );
}
