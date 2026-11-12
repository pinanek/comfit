(setting
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(if_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(else_if_clause
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(else_clause
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(value
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(function_call
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(attribute
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(attribute
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(dependency_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(interpolation
  "{{" @rainbow.bracket
  "}}" @rainbow.bracket) @rainbow.scope
