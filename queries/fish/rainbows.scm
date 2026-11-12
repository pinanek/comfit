(begin_statement
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(brace_expansion
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(command_substitution
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(list_element_access
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope
