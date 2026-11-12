(array_type
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(function_type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(indexed_field
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(link_reference
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(operator_annotation
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(positional_field
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(table_literal_type
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(table_literal_type
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(table_type
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(tuple_type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
