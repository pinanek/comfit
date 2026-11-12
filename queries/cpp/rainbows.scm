[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "[["
  "]]"
] @rainbow.bracket

[
  (preproc_params "(" ")")
  (preproc_defined "(" ")")
  (attribute_specifier "(" ")")
  (attribute_declaration "[[" "]]")
  (ms_declspec_modifier "(" ")")
  (declaration_list "{" "}")
  (parenthesized_declarator "(" ")")
  (abstract_parenthesized_declarator "(" ")")
  (array_declarator "[" "]")
  (abstract_array_declarator "[" "]")
  (compound_statement "{" "}")
  (alignas_qualifier "(" ")")
  (enumerator_list "{" "}")
  (field_declaration_list "{" "}")
  (parameter_list "(" ")")
  (for_statement "(" ")")
  (cast_expression "(" ")")
  (sizeof_expression "(" ")")
  (alignof_expression "(" ")")
  (offsetof_expression "(" ")")
  (generic_expression "(" ")")
  (gnu_asm_expression "(" ")")
  (gnu_asm_output_operand)
  (gnu_asm_input_operand)
  (argument_list "(" ")")
  (compound_literal_expression "(" ")")
  (parenthesized_expression "(" ")")
  (initializer_list "{" "}")
  (subscript_designator "[" "]")
  (subscript_range_designator "[" "]")
  (decltype "(" ")")
  (explicit_function_specifier "(" ")")
  (structured_binding_declarator "[" "]")
  (noexcept "(" ")")
  (throw_specifier "(" ")")
  (static_assert_declaration "(" ")")
  (for_range_loop "(" ")")
  (expansion_statement "(" ")")
  (condition_clause "(" ")")
  (subscript_argument_list "[" "]")
  (new_declarator "[" "]")
  (delete_expression "[" "]")
  (compound_requirement "{" "}")
  (requirement_seq "{" "}")
  (constraint_conjunction "(" ")")
  (constraint_disjunction "(" ")")
  (requires_clause "(" ")")
  (lambda_capture_specifier "[" "]")
  (fold_expression "(" ")")
] @rainbow.scope

; Angle brackets are delimiters only in these grammar contexts.

(template_parameter_list
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope

(template_argument_list
  "<" @rainbow.bracket
  ">" @rainbow.bracket) @rainbow.scope
