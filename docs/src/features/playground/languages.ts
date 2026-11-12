export const languages = [
  {
    id: "rust",
    name: "Rust",
    sample: `// Local bindings and nested brackets
fn greet(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| {
        format!("Hello, {name}!")
    }).collect()
}

fn main() {
    let names = ["Comfit", "Tiếng Việt", "日本語"];
    for message in greet(&names) {
        println!("{message}");
    }
}
`,
  },
  {
    id: "javascript",
    name: "JavaScript",
    sample: `// Functions, strings, and nested expressions
const people = [
  { name: "Comfit", languages: ["Rust", "JavaScript"] },
  { name: "Tree-sitter", languages: ["Python"] },
];

function greet({ name, languages }) {
  return \`Hello, \${name}! Try \${languages.join(" or ")}.\`;
}

console.log(people.map(greet));
`,
  },
  {
    id: "typescript",
    name: "TypeScript",
    sample: `interface Token {
  text: string;
  highlight: string | null;
  rainbow: number | null;
}

function captures(tokens: readonly Token[]): string[] {
  return tokens.flatMap(({ highlight }) => (
    highlight === null ? [] : [highlight]
  ));
}

const token: Token = {
  text: "hello",
  highlight: "string",
  rainbow: null,
};
console.log(captures([token]));
`,
  },
  {
    id: "python",
    name: "Python",
    sample: `from dataclasses import dataclass

@dataclass
class Language:
    name: str
    aliases: tuple[str, ...]

def describe(language: Language) -> str:
    aliases = ", ".join(language.aliases)
    return f"{language.name} ({aliases})"

languages = [
    Language("Python", ("py",)),
    Language("Rust", ("rs",)),
]
for language in languages:
    print(describe(language))
`,
  },
  {
    id: "html",
    name: "HTML",
    sample: `<!doctype html>
<article class="greeting">
  <h1>Hello, Comfit!</h1>
  <img src="/logo.svg" alt="Comfit" />
  <script type="module">
    const names = ["Comfit", "Tree-sitter"];
    console.log(names.map((name) => \`Hello, \${name}!\`));
  </script>
</article>
`,
  },
  {
    id: "bash",
    name: "Bash",
    sample: `#!/usr/bin/env bash
greet() {
  local name="\${1:-Comfit}"
  printf 'Hello, %s!\\n' "$name"
}

names=("Comfit" "Tree-sitter")
for name in "\${names[@]}"; do
  if [[ -n "$name" ]]; then
    greet "$(printf '%s' "$name")"
  fi
done
`,
  },
  {
    id: "markdown",
    name: "Markdown",
    sample: `# Embedded languages

Comfit highlights **Markdown**, inline \`code\`, and fenced code.

\`\`\`rust
fn main() {
    let names = ["Comfit", "Tree-sitter"];
    println!("{:?}", names);
}
\`\`\`

\`\`\`javascript
const greet = (name) => \`Hello, \${name}!\`;
console.log(greet("Comfit"));
\`\`\`
`,
  },
] as const;

export type LanguageId = typeof languages[number]["id"];
