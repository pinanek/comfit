// Adapted from https://github.com/rstacruz/nprogress/issues/253#issue-2923125828.

const speed = 200;
let status: number | null = null;
let trickleTimer: ReturnType<typeof setTimeout> | undefined;
let removeTimer: ReturnType<typeof setTimeout> | undefined;

function setProgress(value: number): void {
  clearTimeout(removeTimer);
  removeTimer = undefined;
  status = value === 1 ? null : value;

  let progress = document.getElementById("nprogress");
  if (!progress) {
    progress = document.createElement("div");
    progress.id = "nprogress";
    progress.setAttribute("role", "progressbar");
    progress.setAttribute("aria-label", "Loading page");
    progress.innerHTML = "<div class=\"bar\" data-bar><div class=\"peg\"></div></div>";
    document.body.append(progress);
    // Commit the starting position before transitioning to the first value.
    void progress.offsetWidth;
  }

  progress.style.setProperty("--nprogress-speed", `${speed}ms`);
  progress.querySelector<HTMLElement>("[data-bar]")?.style.setProperty("--nprogress-position", `${(value - 1) * 100}%`);

  if (value === 1) {
    clearTimeout(trickleTimer);
    trickleTimer = undefined;
    removeTimer = setTimeout(() => {
      progress.remove();
      removeTimer = undefined;
    }, speed);
  }
}

function trickle(): void {
  trickleTimer = setTimeout(() => {
    trickleTimer = undefined;
    if (status === null) return;
    const amount = status < 0.2 ? 0.1 : status < 0.5 ? 0.04 : status < 0.8 ? 0.02 : 0.005;
    setProgress(Math.min(status + amount, 0.994));
    trickle();
  }, 200);
}

export function startProgress(): void {
  if (typeof document === "undefined") return;
  if (status === null) setProgress(0.08);
  if (trickleTimer === undefined) trickle();
}

export function finishProgress(): void {
  if (typeof document !== "undefined" && status !== null) setProgress(1);
}
