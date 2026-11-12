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

(if_start
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(else_if_start
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(else_start
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(if_end
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(each_start
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(each_end
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(await_start
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(then_start
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(catch_start
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(await_end
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(key_start
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(key_end
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(snippet_start
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(snippet_start
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(snippet_end
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(html_tag
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(const_tag
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(debug_tag
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(render_tag
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
