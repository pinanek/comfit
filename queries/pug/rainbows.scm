(attributes
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(children
  "{{" @rainbow.bracket
  "}}" @rainbow.bracket) @rainbow.scope

(children
  "#{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(mixin_attributes
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(mixin_use
  "(" @rainbow.bracket
  ")" @rainbow.bracket) @rainbow.scope

(pipe
  "{{" @rainbow.bracket
  "}}" @rainbow.bracket) @rainbow.scope

(pipe
  "#{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope

(tag
  "{{" @rainbow.bracket
  "}}" @rainbow.bracket) @rainbow.scope

(tag
  "#{" @rainbow.bracket
  "}" @rainbow.bracket) @rainbow.scope
