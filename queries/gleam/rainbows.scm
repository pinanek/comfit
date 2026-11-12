(arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(binary_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(bit_array_segment_option
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(case
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(case_clause_guard
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(data_constructor_arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(function_parameter_types
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(function_parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(list
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(list_pattern
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(panic
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(record_pattern_arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(record_update
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(target_group
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(todo
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(tuple
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(tuple_pattern
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(tuple_type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(type_arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(type_definition
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(type_parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(unqualified_imports
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
