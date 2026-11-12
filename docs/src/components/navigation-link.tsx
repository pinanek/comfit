"use client";

import "#/styles/navigation-progress.css";

import { finishProgress, startProgress } from "#/lib/nprogress";
import { useEffect } from "react";
import { Link as WakuLink, type LinkProps, useNavigationStatus_UNSTABLE } from "waku/router/client";

const pendingLinks = new Set<symbol>();

function NavigationProgress() {
  const { pending } = useNavigationStatus_UNSTABLE();

  useEffect(() => {
    if (!pending) return;

    const navigation = Symbol();
    pendingLinks.add(navigation);
    startProgress();

    return () => {
      pendingLinks.delete(navigation);
      if (pendingLinks.size === 0) finishProgress();
    };
  }, [pending]);

  return null;
}

export function Link({ children, ...props }: LinkProps) {
  return (
    <WakuLink {...props}>
      <NavigationProgress />
      {children}
    </WakuLink>
  );
}
