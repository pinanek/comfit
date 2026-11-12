(expression
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(new_index
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(function_call
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(unary_operation
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(binary_operation
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(block
  (block_start) @rainbow.bracket
  (block_end) @rainbow.bracket) @rainbow.scope

(object
  (object_start) @rainbow.bracket
  (object_end) @rainbow.bracket) @rainbow.scope

(tuple
  (tuple_start) @rainbow.bracket
  (tuple_end) @rainbow.bracket) @rainbow.scope

(for_tuple_expr
  (tuple_start) @rainbow.bracket
  (tuple_end) @rainbow.bracket) @rainbow.scope

(for_object_expr
  (object_start) @rainbow.bracket
  (object_end) @rainbow.bracket) @rainbow.scope

(template_interpolation
  (template_interpolation_start) @rainbow.bracket
  (template_interpolation_end) @rainbow.bracket) @rainbow.scope

(template_for_start
  (template_directive_start) @rainbow.bracket
  (template_directive_end) @rainbow.bracket) @rainbow.scope

(template_for_end
  (template_directive_start) @rainbow.bracket
  (template_directive_end) @rainbow.bracket) @rainbow.scope

(template_if_intro
  (template_directive_start) @rainbow.bracket
  (template_directive_end) @rainbow.bracket) @rainbow.scope

(template_else_intro
  (template_directive_start) @rainbow.bracket
  (template_directive_end) @rainbow.bracket) @rainbow.scope

(template_if_end
  (template_directive_start) @rainbow.bracket
  (template_directive_end) @rainbow.bracket) @rainbow.scope
