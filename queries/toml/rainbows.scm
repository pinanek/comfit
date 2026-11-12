(table
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(table_array_element
  "[[" @rainbow.bracket
  "]]" @rainbow.bracket) @rainbow.scope

(array
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(inline_table
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
