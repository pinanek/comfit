(link_label
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

((link_destination
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope
  (#match? @rainbow.scope "^<"))

((link_title
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
  (#match? @rainbow.scope "^\\("))
