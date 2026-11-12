//! Match TextMate selectors against representative Comfit capture stacks.

/// Representative scope stacks for each Comfit capture, ordered by preferred alternative.
pub const CAPTURES: &[(&str, &[&str])] = &[
    (
        "variable",
        &["variable.other.readwrite", "variable.other", "variable"],
    ),
    (
        "variable.builtin",
        &[
            "variable.language.this",
            "variable.language",
            "support.variable",
        ],
    ),
    (
        "variable.parameter",
        &[
            "variable.parameter",
            "meta.function.parameters variable.other",
        ],
    ),
    (
        "variable.parameter.builtin",
        &["variable.parameter.builtin"],
    ),
    (
        "variable.member",
        &[
            "variable.other.property",
            "variable.object.property",
            "variable.other.member",
            "meta.object.member",
            "meta.property.object",
        ],
    ),
    (
        "constant",
        &[
            "variable.other.constant",
            "entity.name.constant",
            "constant.other",
            "constant",
        ],
    ),
    (
        "constant.builtin",
        &["constant.language", "support.constant"],
    ),
    ("constant.macro", &["entity.name.constant.preprocessor"]),
    (
        "module",
        &[
            "entity.name.namespace",
            "entity.name.module",
            "entity.name.scope-resolution",
        ],
    ),
    ("module.builtin", &["support.module"]),
    ("label", &["entity.name.label"]),
    (
        "string",
        &[
            "string.quoted.double",
            "string",
            "punctuation.definition.string",
        ],
    ),
    ("string.documentation", &["string.quoted.docstring"]),
    ("string.regexp", &["string.regexp"]),
    (
        "string.escape",
        &[
            "constant.character.escape",
            "constant.character.string.escape",
        ],
    ),
    ("string.special", &["string.other"]),
    ("string.special.symbol", &["constant.other.symbol"]),
    ("string.special.path", &["entity.name.filename"]),
    ("string.special.url", &["markup.underline.link"]),
    (
        "character",
        &["string.quoted.single.char", "constant.character"],
    ),
    (
        "boolean",
        &[
            "constant.language.boolean",
            "constant.language.true",
            "constant.language.false",
        ],
    ),
    ("number", &["constant.numeric"]),
    ("number.float", &["constant.numeric.float"]),
    (
        "type",
        &[
            "entity.name.type",
            "entity.name.class",
            "entity.name.struct",
            "entity.name.enum",
            "entity.name.trait",
            "entity.name.union",
            "entity.other.inherited-class",
            "meta.type",
        ],
    ),
    (
        "type.builtin",
        &["support.type.primitive", "support.type", "support.class"],
    ),
    ("type.definition", &["entity.name.type.definition"]),
    (
        "attribute",
        &[
            "meta.annotation",
            "meta.decorator",
            "meta.annotation variable.function",
            "meta.annotation variable.annotation.function",
            "punctuation.definition.annotation",
            "punctuation.decorator",
        ],
    ),
    (
        "attribute.builtin",
        &["support.attribute.builtin", "support.token.decorator"],
    ),
    (
        "property",
        &[
            "meta.property-name",
            "support.type.property-name.json",
            "support.type.property-name.css",
            "support.type.property-name.toml",
            "support.type.property-name",
        ],
    ),
    ("function", &["entity.name.function", "variable.function"]),
    (
        "function.builtin",
        &["support.function.builtin", "support.function"],
    ),
    (
        "function.call",
        &[
            "meta.function-call entity.name.function.call",
            "meta.function.call entity.name.function.call",
            "entity.name.function.call",
        ],
    ),
    (
        "function.macro",
        &["entity.name.function.macro", "support.macro"],
    ),
    ("function.method", &["entity.name.function.method"]),
    (
        "function.method.call",
        &[
            "meta.function-call.method entity.name.function.method.call",
            "meta.method-call entity.name.function.method.call",
            "meta.function.call entity.name.function.method.call",
        ],
    ),
    (
        "constructor",
        &["meta.function-call.constructor entity.name.type"],
    ),
    ("operator", &["keyword.operator"]),
    ("keyword", &["keyword.control", "keyword"]),
    ("keyword.coroutine", &["keyword.control.coroutine"]),
    (
        "keyword.function",
        &["storage.type.function", "keyword.declaration.function"],
    ),
    ("keyword.operator", &["keyword.operator.word"]),
    ("keyword.import", &["keyword.control.import"]),
    ("keyword.type", &["storage.type"]),
    ("keyword.modifier", &["storage.modifier"]),
    ("keyword.repeat", &["keyword.control.loop"]),
    ("keyword.return", &["keyword.control.return"]),
    ("keyword.debug", &["keyword.control.debug"]),
    ("keyword.exception", &["keyword.control.exception"]),
    ("keyword.conditional", &["keyword.control.conditional"]),
    (
        "keyword.directive",
        &[
            "keyword.control.directive",
            "keyword.other.preprocessor",
            "comment.line.shebang",
            "punctuation.definition.directive",
        ],
    ),
    (
        "punctuation.delimiter",
        &[
            "punctuation.separator",
            "punctuation.terminator",
            "punctuation.accessor",
            "punctuation.colon",
        ],
    ),
    (
        "punctuation.bracket",
        &[
            "punctuation.section",
            "punctuation.brackets",
            "punctuation.definition.group",
        ],
    ),
    (
        "punctuation.special",
        &[
            "punctuation.section.interpolation",
            "punctuation.definition.interpolation",
            "punctuation.definition.template-expression",
        ],
    ),
    ("comment", &["comment", "punctuation.definition.comment"]),
    (
        "comment.documentation",
        &["comment.block.documentation", "comment.line.documentation"],
    ),
    ("comment.error", &["comment.error"]),
    ("comment.warning", &["comment.warning"]),
    ("comment.todo", &["comment.todo"]),
    ("comment.note", &["comment.note"]),
    ("markup.strong", &["markup.bold", "strong"]),
    ("markup.italic", &["markup.italic", "emphasis"]),
    ("markup.strikethrough", &["markup.strikethrough"]),
    ("markup.underline", &["markup.underline"]),
    ("markup.heading", &["markup.heading", "entity.name.section"]),
    (
        "markup.heading.1",
        &[
            "markup.heading.1.markdown",
            "markup.heading.atx.1.mdx",
            "heading.1.markdown",
            "markup.heading.1",
            "markup.heading.atx.1",
            "heading.1",
        ],
    ),
    (
        "markup.heading.2",
        &[
            "markup.heading.2.markdown",
            "markup.heading.atx.2.mdx",
            "heading.2.markdown",
            "markup.heading.2",
            "markup.heading.atx.2",
            "heading.2",
        ],
    ),
    (
        "markup.heading.3",
        &[
            "markup.heading.3.markdown",
            "markup.heading.atx.3.mdx",
            "heading.3.markdown",
            "markup.heading.3",
            "markup.heading.atx.3",
            "heading.3",
        ],
    ),
    (
        "markup.heading.4",
        &[
            "markup.heading.4.markdown",
            "markup.heading.atx.4.mdx",
            "heading.4.markdown",
            "markup.heading.4",
            "markup.heading.atx.4",
            "heading.4",
        ],
    ),
    (
        "markup.heading.5",
        &[
            "markup.heading.5.markdown",
            "markup.heading.atx.5.mdx",
            "heading.5.markdown",
            "markup.heading.5",
            "markup.heading.atx.5",
            "heading.5",
        ],
    ),
    (
        "markup.heading.6",
        &[
            "markup.heading.6.markdown",
            "markup.heading.atx.6.mdx",
            "heading.6.markdown",
            "markup.heading.6",
            "markup.heading.atx.6",
            "heading.6",
        ],
    ),
    (
        "markup.quote",
        &["markup.quote", "punctuation.definition.quote"],
    ),
    (
        "markup.link",
        &["markup.link", "punctuation.definition.link"],
    ),
    ("markup.link.label", &["markup.link.label"]),
    (
        "markup.link.url",
        &["markup.underline.link", "markup.link.url"],
    ),
    (
        "markup.raw",
        &[
            "markup.inline.raw.string.markdown",
            "markup.inline.raw",
            "markup.raw",
        ],
    ),
    (
        "markup.raw.block",
        &[
            "markup.raw.block.markdown",
            "markup.fenced_code.block.markdown",
            "markup.raw.block",
            "markup.fenced_code.block",
        ],
    ),
    (
        "markup.list",
        &[
            "markup.list.bullet",
            "markup.list",
            "punctuation.definition.list.begin.markdown",
            "punctuation.definition.list",
        ],
    ),
    ("markup.list.checked", &["markup.list.checked"]),
    ("markup.list.unchecked", &["markup.list.unchecked"]),
    (
        "diff.plus",
        &[
            "markup.inserted.diff",
            "markup.inserted",
            "punctuation.definition.inserted",
        ],
    ),
    (
        "diff.minus",
        &[
            "markup.deleted.diff",
            "markup.deleted",
            "punctuation.definition.deleted",
        ],
    ),
    (
        "diff.delta",
        &[
            "markup.changed.diff",
            "markup.changed",
            "punctuation.definition.changed",
        ],
    ),
    ("tag", &["entity.name.tag"]),
    ("tag.builtin", &["entity.name.tag.html"]),
    ("tag.attribute", &["entity.other.attribute-name"]),
    ("tag.delimiter", &["punctuation.definition.tag"]),
];

/// Match a selector against a scope stack and rank matches from the innermost scope outward.
///
/// Returns `None` for unsupported selector syntax or a selector that does not match.
pub fn specificity(selector: &str, stack: &str) -> Option<Vec<(usize, usize)>> {
    let selector: Vec<_> = selector.split_whitespace().collect();
    let stack: Vec<_> = stack.split_whitespace().collect();

    if selector.is_empty() || !selector.iter().all(|scope| valid(scope)) {
        return None;
    }

    let mut end = stack.len();
    let mut specificity = Vec::new();

    for scope in selector.into_iter().rev() {
        let index = stack[..end]
            .iter()
            .rposition(|candidate| matches(candidate, scope))?;
        specificity.push((index, scope.split('.').count()));
        end = index;
    }

    Some(specificity)
}

fn matches(scope: &str, prefix: &str) -> bool {
    scope == prefix
        || scope
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('.'))
}

fn valid(scope: &str) -> bool {
    scope.split('.').all(|part| {
        !part.is_empty()
            && part
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    })
}
