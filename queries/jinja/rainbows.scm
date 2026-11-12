; inherits: jinja_inline

(statement
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(if_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(elif_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(else_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(for_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(block_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(with_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(filter_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(macro_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(call_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(set_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(trans_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(pluralize_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(autoescape_block
  [ "{%" "{%-" "{%+" ] @rainbow.bracket
  [ "%}" "-%}" "+%}" ] @rainbow.bracket) @rainbow.scope

(render_expression
  [ "{{" "{{-" "{{+" ] @rainbow.bracket
  [ "}}" "-}}" "+}}" ] @rainbow.bracket) @rainbow.scope
