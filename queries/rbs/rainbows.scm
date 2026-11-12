(attribute_member
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(block
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(method_type_parameters
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(module_type_parameters
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(parameters
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(record_type
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(self_type_binding
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(singleton_type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(tuple_type
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(type
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(type_arguments
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope
