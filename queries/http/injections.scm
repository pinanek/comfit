; Comments
((comment) @injection.content
  (#set! injection.language "comment"))

; Body
((json_body) @injection.content
  (#set! injection.language "json"))

; Compact JSON is parsed as raw_body by the grammar.
((_
  (header
    name: (_) @_header
    value: (_) @_content_type)
  (raw_body) @injection.content)
  (#match? @_header "^[cC][oO][nN][tT][eE][nN][tT]-[tT][yY][pP][eE]$")
  (#match? @_content_type "^[aA][pP][pP][lL][iI][cC][aA][tT][iI][oO][nN]/([A-Za-z0-9.-]+\\+)?[jJ][sS][oO][nN]([; \\t]|$)")
  (#set! injection.language "json"))

((xml_body) @injection.content
  (#set! injection.language "xml"))

((graphql_data) @injection.content
  (#set! injection.language "graphql"))

; Script (default to javascript)
((comment
  name: (_) @_name
  (#eq? @_name "lang")
  value: (_) @injection.language)?
  .
  (_
    (script) @injection.content
    (#offset! @injection.content 0 2 0 -2))
  (#set! injection.language "javascript"))
