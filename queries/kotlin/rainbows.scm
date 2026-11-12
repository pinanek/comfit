(file_annotation
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(primary_constructor
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(class_body
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(delegation_specifier
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(type_parameters
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(anonymous_initializer
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(function_value_parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(context_receiver
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(function_body
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(getter
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(setter
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(secondary_constructor
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(enum_class_body
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(function_type_parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_user_type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(control_structure_body
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(for_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(while_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(do_while_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(indexing_suffix
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(type_arguments
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(value_arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(collection_literal
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(lambda_literal
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(multi_variable_declaration
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(super_expression
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(if_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(when_subject
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(when_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(try_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(catch_block
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(catch_block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(finally_block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(annotation
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(string_literal
  (interpolation_expression_start) @rainbow.bracket
  (interpolation_expression_end) @rainbow.bracket) @rainbow.scope
