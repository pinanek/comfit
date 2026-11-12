(namespace_use_group
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(enum_declaration_list
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(declaration_list
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(property_hook_list
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(use_list
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(visibility_modifier
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(anonymous_function_use_clause
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(formal_parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(disjunctive_normal_form_type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(exit_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(unset_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(declare_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(catch_clause
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(for_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(foreach_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(match_block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(switch_block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(compound_statement
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(class_constant_access_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(cast_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(list_literal
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(list_literal
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(subscript_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(array_creation_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(array_creation_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(member_access_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(member_call_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(nullsafe_member_access_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(nullsafe_member_call_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(scoped_call_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(encapsed_string
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(shell_command_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(dynamic_variable_name
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(heredoc_body
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(attribute_group
  "#[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope
