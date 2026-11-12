[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @rainbow.bracket

[
  (macro_definition)
  (token_tree_pattern)
  (token_repetition_pattern "(" ")")
  (token_tree)
  (token_repetition "(" ")")
  (attribute_item "[" "]")
  (inner_attribute_item "[" "]")
  (declaration_list "{" "}")
  (enum_variant_list "{" "}")
  (field_declaration_list "{" "}")
  (ordered_field_declaration_list "(" ")")
  (use_list "{" "}")
  (parameters "(" ")")
  (visibility_modifier "(" ")")
  (array_type "[" "]")
  (tuple_type "(" ")")
  (unit_type "(" ")")
  (arguments "(" ")")
  (array_expression "[" "]")
  (parenthesized_expression "(" ")")
  (tuple_expression "(" ")")
  (unit_expression "(" ")")
  (field_initializer_list "{" "}")
  (match_block "{" "}")
  (index_expression "[" "]")
  (block "{" "}")
  (tuple_pattern "(" ")")
  (slice_pattern "[" "]")
  (tuple_struct_pattern "(" ")")
  (struct_pattern "{" "}")
] @rainbow.scope

; Angle brackets are delimiters only in these grammar contexts.

(type_parameters
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(bracketed_type
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(for_lifetimes
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(use_bounds
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(type_arguments
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope
