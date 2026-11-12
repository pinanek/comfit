(action
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(character_class
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(host_lang
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(limits
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(parenthesized
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
