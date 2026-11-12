; Adapted from tree-sitter-markdown's inline highlight query.
[
  (code_span)
  (link_title)
] @markup.raw

(emphasis) @markup.italic

(strong_emphasis) @markup.strong

(strikethrough) @markup.strikethrough

[
  (link_destination)
  (uri_autolink)
  (email_autolink)
] @markup.link.url

[
  (link_label)
  (link_text)
  (image_description)
] @markup.link.label

[
  (backslash_escape)
  (hard_line_break)
] @string.escape

[
  (emphasis_delimiter)
  (code_span_delimiter)
] @punctuation.delimiter

(image
  ["!" "[" "]" "(" ")"] @punctuation.delimiter)

(inline_link
  ["[" "]" "(" ")"] @punctuation.delimiter)

(shortcut_link
  ["[" "]"] @punctuation.delimiter)
