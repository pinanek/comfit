(access_call
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(arguments
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(bitstring
  "<<" @rainbow.bracket
  ">>" @rainbow.bracket) @rainbow.scope

(block
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(interpolation
  "#{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(list
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(map
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(sigil
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(sigil
  "[" @rainbow.bracket
  "]" @rainbow.bracket) @rainbow.scope

(sigil
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(tuple
  "{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(unary_operator
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope
