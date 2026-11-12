(begin_block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(end_block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(singleton_method
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(array_pattern
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(array_pattern
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(find_pattern
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(find_pattern
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(hash_pattern
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(hash_pattern
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(hash_pattern
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(parenthesized_pattern
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(expression_reference_pattern
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_statements
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(argument_list
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(destructured_left_assignment
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(interpolation
  "#{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(array
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(hash
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(block_parameters
  "|" @rainbow.bracket
  "|" @rainbow.bracket) @rainbow.scope

(method_parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(destructured_parameter
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(lambda_parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(string_array
  "%w(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(symbol_array
  "%i(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
