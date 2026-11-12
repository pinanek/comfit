(arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(attribute_selector
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(feature_query
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(grid_value
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(interpolation
  "${" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(js_expression
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(keyframe_block_list
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(parenthesized_query
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(parenthesized_value
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(selector_query
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
