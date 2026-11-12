(array
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(attribute_access
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(function_call
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(object
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(order_function
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesis_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(projection
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(select_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
