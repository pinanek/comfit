const names: Record<string, string> = {
  c: "C",
  cpp: "C++",
  javascript: "JavaScript",
  js: "JavaScript",
  jsx: "JSX",
  typescript: "TypeScript",
  ts: "TypeScript",
  tsx: "TSX",
  python: "Python",
  py: "Python",
  rust: "Rust",
  rs: "Rust",
  markdown: "Markdown",
  md: "Markdown",
  html: "HTML",
  css: "CSS",
  json: "JSON",
  sh: "Shell",
  bash: "Bash",
  text: "Plain text",
};

export function languageName(language = "text"): string {
  return names[language] ?? language;
}
