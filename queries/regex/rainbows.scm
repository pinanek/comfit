(anonymous_capturing_group
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(character_class
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(character_class_escape
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(count_quantifier
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(inline_flags_group
  "(?" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(lookaround_assertion
  "(?" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(lookaround_assertion
  "(?<" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(named_capturing_group
  "(?<" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(named_capturing_group
  "(?P<" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(named_group_backreference
  "(?P=" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(non_capturing_group
  "(?:" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
