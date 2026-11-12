; comments and block-comments
((comment) @injection.content
  (#set! injection.language "comment"))

; dynamic invocation like & "$env:ChocolateyInstall\bin\choco.exe"
(command
  (command_invokation_operator) @_operator
  (command_name_expr
    (string_literal) @injection.content)
  (#eq? @_operator "&")
  (#set! injection.language "powershell")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.include-children))

(comparison_expression
  (comparison_operator) @_operator
  (additive_expression
    (multiplicative_expression
      (format_expression
        (range_expression
          (array_literal_expression
            .
            (unary_expression
              (string_literal) @injection.content))))))
  (#match? @_operator "^-[iIcC]?([mM][aA][tT][cC][hH]|[nN][oO][tT][mM][aA][tT][cC][hH]|[rR][eE][pP][lL][aA][cC][eE]|[sS][pP][lL][iI][tT])$")
  (#match? @injection.content "^['\"]")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "regex")
  (#set! injection.include-children))

; switch -regex
(switch_statement
  (switch_parameters
    (switch_parameter) @_parameter)
  (switch_body
    (switch_clauses
      (switch_clause
        (switch_clause_condition
          (string_literal) @injection.content))))
  (#eq? @_parameter "-regex")
  (#set! injection.language "regex")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.include-children))
