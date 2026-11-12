(code_block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(function
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(storageclass
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope
