(function_definition
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(compound_statement
  ["{" "}" "((" "))"] @rainbow.bracket) @rainbow.scope

(subshell
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(test_command
  ["[" "]" "[[" "]]"] @rainbow.bracket) @rainbow.scope

(array
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(subscript
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(expansion
  "${" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(command_substitution
  ["$(" ")" "`" "$`"] @rainbow.bracket) @rainbow.scope

(process_substitution
  ["<(" ">(" ")"] @rainbow.bracket) @rainbow.scope

(arithmetic_expansion
  ["$((" "))" "$[" "]"] @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(c_style_for_statement
  "((" @rainbow.bracket
  "))" @rainbow.bracket) @rainbow.scope

(brace_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
