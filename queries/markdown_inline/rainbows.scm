[
  (inline_link)
  (full_reference_link)
  (collapsed_reference_link)
  (shortcut_link)
  (image)
] @rainbow.scope

(inline_link
  ["[" "]" "(" ")"] @rainbow.bracket)

(full_reference_link
  ["[" "]"] @rainbow.bracket)

(collapsed_reference_link
  ["[" "]"] @rainbow.bracket)

(shortcut_link
  ["[" "]"] @rainbow.bracket)

(image
  ["[" "]" "(" ")"] @rainbow.bracket)

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
