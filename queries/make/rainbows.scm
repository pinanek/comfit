(archive
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(automatic_variable
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(automatic_variable
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(function_call
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(ifeq_directive
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(ifneq_directive
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(shell_function
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(substitution_reference
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(substitution_reference
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(variable_reference
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(variable_reference
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
