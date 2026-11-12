(mustache_statement
  "{{" @rainbow.bracket
  "}}" @rainbow.bracket) @rainbow.scope

(mustache_statement
  "{{~" @rainbow.bracket
  "~}}" @rainbow.bracket) @rainbow.scope

(mustache_statement
  "{{" @rainbow.bracket
  "~}}" @rainbow.bracket) @rainbow.scope

(mustache_statement
  "{{~" @rainbow.bracket
  "}}" @rainbow.bracket) @rainbow.scope

(sub_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

((element_node
  (element_node_start)
  (element_node_end)) @rainbow.scope
  (#set! rainbow.include-children))

(element_node_start
  ["<" ">"] @rainbow.bracket)

(element_node_end
  ["</" ">"] @rainbow.bracket)

((script_element
  (start_tag)
  (end_tag)) @rainbow.scope
  (#set! rainbow.include-children))

(start_tag
  ["<" ">"] @rainbow.bracket)

(end_tag
  ["</" ">"] @rainbow.bracket)

((style_element
  (start_tag)
  (end_tag)) @rainbow.scope
  (#set! rainbow.include-children))

(start_tag
  ["<" ">"] @rainbow.bracket)

(end_tag
  ["</" ">"] @rainbow.bracket)

(element_node_void
  ["<" "/>"] @rainbow.bracket) @rainbow.scope

((block_statement
  (block_statement_start)
  (block_statement_end)) @rainbow.scope
  (#set! rainbow.include-children))

(block_statement_start
  ["{{#" "{{~#" "}}" "~}}"] @rainbow.bracket)

(block_statement_end
  ["{{/" "{{~/" "}}" "~}}"] @rainbow.bracket)
