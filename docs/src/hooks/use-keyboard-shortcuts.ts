"use client";

import { useEffect, useEffectEvent } from "react";
import { defaultKeybindingsHandlerIgnore, type KeybindingsMap, tinykeys } from "tinykeys";

export function useKeyboardShortcuts(
  shortcuts: KeybindingsMap,
  { enabled = true, allowInInputs = false }: { enabled?: boolean; allowInInputs?: boolean } = {},
) {
  const onShortcut = useEffectEvent((binding: string, event: KeyboardEvent) => {
    shortcuts[binding]?.(event);
  });
  const bindings = JSON.stringify(Object.keys(shortcuts).sort());

  useEffect(() => {
    if (!enabled) return;

    const handlers: KeybindingsMap = {};
    for (const binding of JSON.parse(bindings) as string[]) {
      handlers[binding] = (event) => onShortcut(binding, event);
    }

    return tinykeys(window, handlers, {
      ignore: (event) =>
        event.repeat || event.isComposing || (!allowInInputs && defaultKeybindingsHandlerIgnore(event)),
    });
  }, [bindings, enabled, allowInInputs]);
}
