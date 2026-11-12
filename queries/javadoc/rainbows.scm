(description
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(method
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parameter
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(param_tag
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope
