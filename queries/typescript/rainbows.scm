; inherits: javascript

[
  (import_require_clause "(" ")")
  (enum_body "{" "}")
  (template_type "${" "}")
  (lookup_type "[" "]")
  (parenthesized_type "(" ")")
  (object_type "{" "}")
  (index_signature "[" "]")
  (array_type "[" "]")
  (tuple_type "[" "]")
] @rainbow.scope

; Angle brackets are delimiters only in these grammar contexts.

(type_arguments
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(type_parameters
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope
