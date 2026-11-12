(parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(bracket_index_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(table_constructor
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(field
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(attribute
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope
