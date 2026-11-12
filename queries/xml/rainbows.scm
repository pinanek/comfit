(Enumeration
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(Mixed
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(NotationType
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(children
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(doctypedecl
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

((element
  (STag)
  (ETag)) @rainbow.scope
  (#set! rainbow.include-children))

(STag
  ["<" ">"] @rainbow.bracket)

(ETag
  ["</" ">"] @rainbow.bracket)

(EmptyElemTag
  ["<" "/>"] @rainbow.bracket) @rainbow.scope
