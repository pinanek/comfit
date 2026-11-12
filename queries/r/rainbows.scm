(arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(arguments
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(braced_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(for_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(if_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(while_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
