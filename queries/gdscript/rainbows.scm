(arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(array
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(dictionary
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(enumerator_list
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(subscript_arguments
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope
