(list_literal
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(tuple_literal
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(dict_literal
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(primary_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(primary_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(function_call
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(inline_trans
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(call_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
