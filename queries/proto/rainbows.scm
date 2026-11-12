(option
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(enum_body
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(enum_field
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(enum_value_option
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(message_body
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(group
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(field
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(field_option
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(oneof
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(oneof_field
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(map_field
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(map_field
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(extensions
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(service
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(rpc
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(rpc
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(block_lit
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(block_lit
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(array_lit
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(extension_name
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope
