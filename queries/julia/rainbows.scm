(argument_list
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(comprehension_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(curly_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(export_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(import_alias
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(import_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(matrix_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(public_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(quote_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(selected_import
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(string_interpolation
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(tuple_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(using_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(vector_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope
