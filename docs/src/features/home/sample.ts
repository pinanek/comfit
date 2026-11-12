type SampleToken = readonly [text: string, capture?: string, rainbow?: number];

// Comfit captures for this fixed Markdown sample, with the Rust injection including the fence content’s children.
export const sampleLines: readonly (readonly SampleToken[])[] = [
  [["# Hello, Comfit", "markup.heading.1"]],
  [],
  [["```", "markup.raw.block"], ["rust", "label"]],
  [
    ["fn", "keyword.function"],
    [" ", "markup.raw.block"],
    ["main", "function"],
    ["(", "punctuation.bracket", 0],
    [")", "punctuation.bracket", 0],
    [" ", "markup.raw.block"],
    ["{", "punctuation.bracket", 0],
  ],
  [
    ["    ", "markup.raw.block"],
    ["let", "keyword"],
    [" ", "markup.raw.block"],
    ["name", "variable"],
    [" ", "markup.raw.block"],
    ["=", "operator"],
    [" ", "markup.raw.block"],
    ["\"Comfit\"", "string"],
    [";", "punctuation.delimiter"],
  ],
  [],
  [
    ["    ", "markup.raw.block"],
    ["println", "function.macro"],
    ["!", "function.macro"],
    ["(", "punctuation.bracket", 0],
    ["\"{:?}\"", "string"],
    [",", "punctuation.delimiter"],
    [" ", "markup.raw.block"],
    ["[", "punctuation.bracket", 1],
    ["name", "variable"],
    ["]", "punctuation.bracket", 1],
    [")", "punctuation.bracket", 0],
    [";", "punctuation.delimiter"],
  ],
  [["}", "punctuation.bracket", 0]],
  [["```", "markup.raw.block"]],
];

export const sampleSource = sampleLines.map((line) => line.map(([text]) => text).join("")).join("\n");
