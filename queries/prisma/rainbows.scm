(statement_block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(enum_block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(array
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope
