(argument_list
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(array_expression
  "@(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(attribute
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(attribute
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(class_method_definition
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(class_method_definition
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(class_statement
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(do_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(element_access
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(elseif_clause
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(enum_statement
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(for_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(foreach_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(function_parameter_declaration
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(function_statement
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(hash_literal_expression
  "@{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(if_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(param_block
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(script_block_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(statement_block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(switch_body
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(switch_condition
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(type_literal
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(while_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
