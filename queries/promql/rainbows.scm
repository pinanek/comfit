(binary_expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(binary_grouping
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(function_args
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(grouping
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(label_selectors
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(query
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(range_selection
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(subquery
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(subquery_range_selection
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope
