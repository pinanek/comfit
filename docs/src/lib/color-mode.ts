export const COLOR_MODE_KEY = "comfit-color-mode";

export const COLOR_MODE_SCRIPT = `
(() => {
  const root = document.documentElement;
  let storedMode;

  try {
    storedMode = localStorage.getItem("${COLOR_MODE_KEY}");
  } catch {}

  const mode = storedMode === "light" || storedMode === "dark"
    ? storedMode
    : window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";

  root.classList.toggle("dark", mode === "dark");
  root.style.colorScheme = mode;
})();
`;
