(attrset_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(formals
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(inherit_from
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(interpolation
  "${" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(let_attrset_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(list_expression
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(parenthesized_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(rec_attrset_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
