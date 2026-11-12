(group
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(call
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(content
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(import
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(label
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope
