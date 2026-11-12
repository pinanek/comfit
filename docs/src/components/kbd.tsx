import styles from "./kbd.module.css";

const labels: Record<string, string> = {
  $mod: "⌘",
  Meta: "⌘",
  Control: "Ctrl",
  Shift: "Shift",
  Alt: "⌥",
};

export function Kbd({ shortcut }: { shortcut: string }) {
  return (
    <span className={styles.keys} aria-hidden="true">
      {shortcut.split("+").map((key) => (
        <kbd className={styles.key} key={key}>{labels[key] ?? key.replace(/^Key/, "")}</kbd>
      ))}
    </span>
  );
}
