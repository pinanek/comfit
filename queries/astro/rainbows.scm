([
  (element)
  (script_element)
  (style_element)
] @rainbow.scope
  (#set! rainbow.include-children))

(start_tag
  ["<" ">"] @rainbow.bracket)

(end_tag
  ["</" ">"] @rainbow.bracket)

(self_closing_tag
  "<" @rainbow.bracket
  "/>" @rainbow.bracket) @rainbow.scope

(doctype
  "<!" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(attribute_interpolation
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(html_interpolation
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
