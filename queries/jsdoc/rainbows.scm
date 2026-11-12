(array_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(inline_tag
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(optional_identifier
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(tag
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
