(directive
  "<%" @rainbow.bracket
  "%>" @rainbow.bracket) @rainbow.scope

(directive
  "<%=" @rainbow.bracket
  "%>" @rainbow.bracket) @rainbow.scope

(directive
  "<%%" @rainbow.bracket
  "%>" @rainbow.bracket) @rainbow.scope

(directive
  "<%%=" @rainbow.bracket
  "%>" @rainbow.bracket) @rainbow.scope

(expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

((tag
  (start_tag)
  (end_tag)) @rainbow.scope
  (#set! rainbow.include-children))

(start_tag
  ["<" ">"] @rainbow.bracket)

(end_tag
  ["</" ">"] @rainbow.bracket)

(self_closing_tag
  ["<" "/>"] @rainbow.bracket) @rainbow.scope

((component
  (start_component)
  (end_component)) @rainbow.scope
  (#set! rainbow.include-children))

(start_component
  ["<" ">"] @rainbow.bracket)

(end_component
  ["</" ">"] @rainbow.bracket)

(self_closing_component
  ["<" "/>"] @rainbow.bracket) @rainbow.scope

((slot
  (start_slot)
  (end_slot)) @rainbow.scope
  (#set! rainbow.include-children))

(start_slot
  ["<:" ">"] @rainbow.bracket)

(end_slot
  ["</:" ">"] @rainbow.bracket)

(self_closing_slot
  ["<:" "/>"] @rainbow.bracket) @rainbow.scope
