(import_collection
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(diagnostic_directive
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(attribute
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(compound_global_decl
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(struct_body
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(call_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(indexing_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(paren_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(compound_statement
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(func_call_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(switch_body
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(for_statement
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(function_header
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(template_list
  (template_args_start) @rainbow.bracket
  (template_args_end) @rainbow.bracket) @rainbow.scope
