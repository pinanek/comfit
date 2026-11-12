(array_type
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(array_type
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(disjunctive_normal_form_type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(generic_type
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(inline_tag
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(primitive_type
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope
