(call_syntax_arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(effective_address
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(grouped_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(preproc_function_def_parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(pseudo_instruction_dx
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
