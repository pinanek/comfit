(returns
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(parameter_parens
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parameter_bracks
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(collection_type
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(list_type
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(composite_type
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(flag_capsule
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(ctrl_match
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(command_list
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(expr_unary
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(expr_parenthesized
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(val_nothing
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(val_binary
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(expr_interpolated
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(val_list
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(val_record
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(val_table
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(val_closure
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
