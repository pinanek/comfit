; inherits: javascript

((jsx_element) @rainbow.scope
  (#set! rainbow.include-children))

(jsx_opening_element
  ["<" ">"] @rainbow.bracket)

(jsx_closing_element
  ["</" ">"] @rainbow.bracket)

(jsx_self_closing_element
  "<" @rainbow.bracket
  "/>" @rainbow.bracket) @rainbow.scope

(jsx_expression "{" "}") @rainbow.scope
