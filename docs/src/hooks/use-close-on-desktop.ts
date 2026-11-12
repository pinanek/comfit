"use client";

import { useEffect, useEffectEvent } from "react";

export function useCloseOnDesktop(open: boolean, onClose: () => void) {
  const close = useEffectEvent(onClose);

  useEffect(() => {
    if (!open) return;

    const desktop = window.matchMedia("(min-width: 56rem)");
    const closeOnDesktop = () => {
      if (desktop.matches) close();
    };
    closeOnDesktop();
    desktop.addEventListener("change", closeOnDesktop);

    return () => desktop.removeEventListener("change", closeOnDesktop);
  }, [open]);
}
