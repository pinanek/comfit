(expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(expression_value
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

((block
  (start_block)
  (end_block)) @rainbow.scope
  (#set! rainbow.include-children))

(start_block
  ["{#" "}"] @rainbow.bracket)

(end_block
  ["{/" "}"] @rainbow.bracket)
