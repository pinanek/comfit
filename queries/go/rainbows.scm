(import_spec_list
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(const_declaration
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(var_spec_list
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(type_parameter_list
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(parameter_list
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(type_declaration
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(type_arguments
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(array_type
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(implicit_length_array_type
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(slice_type
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(field_declaration_list
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(interface_type
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(map_type
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(expression_switch_statement
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(type_switch_statement
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(select_statement
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(argument_list
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(index_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(slice_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(type_assertion_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(type_conversion_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(type_instantiation_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(literal_value
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
