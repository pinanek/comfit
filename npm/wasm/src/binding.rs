//! Bindings for web-tree-sitter with shared runtime initialization.

// Adapted from ast-grep/tree-sitter-wasm:
// https://github.com/ast-grep/tree-sitter-wasm/blob/5a18d45612fdb6868ee8cd6de1e5335ce588493c/src/lib.rs
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
// See ../LICENSE.binding. Query metadata bindings are extended for Comfit.

use std::cell::RefCell;

use js_sys::{Array, JsString, Object, Promise, Reflect, Uint8Array};
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;

thread_local! {
    static INITIALIZATION: RefCell<Option<Promise>> = const { RefCell::new(None) };
}

/// Shared initialization for the web-tree-sitter runtime.
pub struct TreeSitter;

impl TreeSitter {
    /// Initialize the runtime from bytes, sharing concurrent attempts.
    ///
    /// # Errors
    ///
    /// Returns the runtime rejection if initialization fails. Later calls may retry.
    pub async fn init(wasm: Uint8Array) -> Result<(), JsValue> {
        let promise = INITIALIZATION.with(|pending| {
            pending
                .borrow_mut()
                .get_or_insert_with(|| {
                    let options = Object::new();
                    Reflect::set(&options, &"wasmBinary".into(), &wasm).unwrap();
                    Parser::init(&options)
                })
                .clone()
        });

        if let Err(error) = JsFuture::from(promise.clone()).await {
            INITIALIZATION.with(|pending| {
                let mut pending = pending.borrow_mut();

                if pending
                    .as_ref()
                    .is_some_and(|current| Object::is(current, &promise))
                {
                    *pending = None;
                }
            });
            return Err(error);
        }

        Ok(())
    }
}

#[wasm_bindgen(module = "web-tree-sitter")]
extern "C" {
    /// A grammar loaded by web-tree-sitter for parsing and query compilation.
    #[derive(Clone)]
    pub type Language;

    #[wasm_bindgen(static_method_of = Language, js_name = load)]
    fn load_raw(bytes: &Uint8Array) -> Promise;

    /// A runtime parser that requires explicit resource cleanup.
    pub type Parser;

    #[wasm_bindgen(static_method_of = Parser)]
    fn init(options: &Object) -> Promise;

    /// Create a parser after runtime initialization.
    ///
    /// # Errors
    ///
    /// Returns the JavaScript exception if parser creation fails.
    #[wasm_bindgen(constructor, catch)]
    pub fn new() -> Result<Parser, JsValue>;

    /// Select the grammar for subsequent parses, or clear it with `None`.
    ///
    /// # Errors
    ///
    /// Returns the runtime exception if the grammar is rejected.
    #[wasm_bindgen(method, catch, js_name = setLanguage)]
    pub fn set_language(this: &Parser, language: Option<&Language>) -> Result<(), JsValue>;

    /// Parse a JavaScript string, optionally reusing a previous tree and included ranges.
    ///
    /// Indices and columns use UTF-16 code units.
    ///
    /// # Errors
    ///
    /// Returns the runtime exception if parsing fails. The runtime may also return no tree.
    #[wasm_bindgen(method, catch, js_name = parse)]
    pub fn parse_with_string(
        this: &Parser,
        source: &str,
        previous: Option<&Tree>,
        options: Option<&ParseOptions>,
    ) -> Result<Option<Tree>, JsValue>;

    /// Release parser resources. Do not use the parser after deletion.
    #[wasm_bindgen(method)]
    pub fn delete(this: &Parser);

    /// A compiled runtime query that requires explicit resource cleanup.
    pub type Query;

    /// Compile query source against a loaded grammar.
    ///
    /// # Errors
    ///
    /// Returns the runtime exception for invalid queries or compilation failures.
    #[wasm_bindgen(constructor, catch)]
    pub fn new(language: &Language, source: &str) -> Result<Query, JsValue>;

    /// Return capture names in the runtime query's capture-index order.
    #[wasm_bindgen(method, getter, js_name = captureNames)]
    pub fn capture_names(this: &Query) -> Box<[JsValue]>;

    /// Return property settings indexed by query pattern.
    #[wasm_bindgen(method, getter, js_name = setProperties)]
    pub fn settings(this: &Query) -> Array;

    /// Return negative property predicates indexed by query pattern.
    #[wasm_bindgen(method, getter, js_name = refutedProperties)]
    pub fn refuted_properties(this: &Query) -> Array;

    /// Return positive property predicates indexed by query pattern.
    #[wasm_bindgen(method, getter, js_name = assertedProperties)]
    pub fn asserted_properties(this: &Query) -> Array;

    /// Return the number of compiled query patterns.
    #[wasm_bindgen(method, js_name = patternCount)]
    pub fn pattern_count(this: &Query) -> u32;

    /// Return the runtime predicate descriptions for one pattern index.
    #[wasm_bindgen(method, js_name = predicatesForPattern)]
    pub fn predicates_for_pattern(this: &Query, index: u32) -> Box<[JsValue]>;

    /// Collect complete query matches from a node with optional runtime settings.
    ///
    /// # Errors
    ///
    /// Returns the runtime exception if query execution fails.
    #[wasm_bindgen(method, catch)]
    pub fn matches(
        this: &Query,
        node: &SyntaxNode,
        options: Option<&Object>,
    ) -> Result<Array, JsValue>;

    /// Return whether the last query exceeded its match limit.
    #[wasm_bindgen(method, js_name = didExceedMatchLimit)]
    pub fn did_exceed_match_limit(this: &Query) -> bool;

    /// Release compiled-query resources. Do not use the query after deletion.
    #[wasm_bindgen(method)]
    pub fn delete(this: &Query);
}

impl Language {
    /// Load a grammar from explicit WASM bytes.
    ///
    /// # Errors
    ///
    /// Returns the runtime rejection if the grammar cannot be loaded.
    pub async fn load_bytes(bytes: &Uint8Array) -> Result<Self, JsValue> {
        Ok(JsFuture::from(Self::load_raw(bytes))
            .await?
            .unchecked_into())
    }
}

#[wasm_bindgen]
extern "C" {
    /// A runtime predicate operator and its ordered operands.
    pub type PredicateResult;

    /// Return the runtime predicate operator name.
    #[wasm_bindgen(method, getter)]
    pub fn operator(this: &PredicateResult) -> JsString;

    /// Return the predicate's capture or literal operands in source order.
    #[wasm_bindgen(method, getter)]
    pub fn operands(this: &PredicateResult) -> Box<[JsValue]>;

    /// A capture name or literal value supplied to a runtime predicate.
    pub type PredicateOperand;

    /// Return the capture name for a capture operand.
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &PredicateOperand) -> JsString;

    /// Return the literal value for a string operand.
    #[wasm_bindgen(method, getter)]
    pub fn value(this: &PredicateOperand) -> JsString;

    /// Return the operand type, such as `capture` or `string`.
    #[wasm_bindgen(method, getter, js_name = type)]
    pub fn type_(this: &PredicateOperand) -> JsString;

    /// One runtime query match, preserving capture relationships.
    pub type QueryMatch;

    /// Return the zero-based query pattern index for this match.
    #[wasm_bindgen(method, getter, js_name = patternIndex)]
    pub fn pattern(this: &QueryMatch) -> u32;

    /// Return captured nodes in runtime match order.
    #[wasm_bindgen(method, getter)]
    pub fn captures(this: &QueryMatch) -> Box<[JsValue]>;

    /// A runtime capture name paired with its syntax node.
    pub type QueryCapture;

    /// Return the query capture name.
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &QueryCapture) -> JsString;

    /// Return the captured syntax node.
    #[wasm_bindgen(method, getter)]
    pub fn node(this: &QueryCapture) -> SyntaxNode;

    /// A node handle whose indices refer to UTF-16 units in the parsed source.
    pub type SyntaxNode;

    /// Return the runtime node ID used to relate captures within one parsed tree.
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &SyntaxNode) -> u32;

    /// Return the immediate parent, or `None` for a root node.
    #[wasm_bindgen(method, getter)]
    pub fn parent(this: &SyntaxNode) -> Option<SyntaxNode>;

    /// Return immediate children in source order.
    #[wasm_bindgen(method, getter)]
    pub fn children(this: &SyntaxNode) -> Box<[JsValue]>;

    /// Return the inclusive start index in UTF-16 code units.
    #[wasm_bindgen(method, getter, js_name = startIndex)]
    pub fn start_index(this: &SyntaxNode) -> u32;

    /// Return the exclusive end index in UTF-16 code units.
    #[wasm_bindgen(method, getter, js_name = endIndex)]
    pub fn end_index(this: &SyntaxNode) -> u32;

    /// Return whether the grammar classifies this node as named.
    #[wasm_bindgen(method, getter, js_name = isNamed)]
    pub fn is_named(this: &SyntaxNode) -> bool;

    /// Return the grammar's node-kind name.
    #[wasm_bindgen(method, getter, js_name = type)]
    pub fn type_(this: &SyntaxNode) -> JsString;

    /// A parsed runtime tree that requires explicit resource cleanup.
    pub type Tree;

    /// Return the parsed tree's root node.
    #[wasm_bindgen(method, getter, js_name = rootNode)]
    pub fn root_node(this: &Tree) -> SyntaxNode;

    /// Apply UTF-16 edit coordinates before reusing a tree in a parse.
    #[wasm_bindgen(method, catch)]
    pub fn edit(this: &Tree, edit: &Object) -> Result<(), JsValue>;

    /// Release tree resources. Do not use the tree or its nodes after deletion.
    #[wasm_bindgen(method)]
    pub fn delete(this: &Tree);

    /// A zero-based row and UTF-16 column supplied to runtime parse options.
    #[wasm_bindgen(extends = Object)]
    pub type Point;

    /// A half-open runtime range with UTF-16 indices and row/column endpoints.
    #[wasm_bindgen(extends = Object)]
    pub type Range;

    /// Runtime parsing options, including optional included source ranges.
    #[wasm_bindgen(extends = Object)]
    pub type ParseOptions;
}

impl Point {
    /// Build a runtime point from a zero-based row and UTF-16 column.
    pub fn new(row: u32, column: u32) -> Self {
        let point = Object::new();
        Reflect::set(&point, &"row".into(), &row.into()).unwrap();
        Reflect::set(&point, &"column".into(), &column.into()).unwrap();
        point.unchecked_into()
    }
}

impl Range {
    /// Build a runtime range from corresponding UTF-16 indices and point endpoints.
    pub fn new(start: &Point, end: &Point, start_index: u32, end_index: u32) -> Self {
        let range = Object::new();
        Reflect::set(&range, &"startPosition".into(), start).unwrap();
        Reflect::set(&range, &"endPosition".into(), end).unwrap();
        Reflect::set(&range, &"startIndex".into(), &start_index.into()).unwrap();
        Reflect::set(&range, &"endIndex".into(), &end_index.into()).unwrap();
        range.unchecked_into()
    }
}

impl ParseOptions {
    /// Build parsing options, omitting included ranges to use the runtime defaults.
    pub fn new(ranges: Option<&Array>) -> Self {
        let options = Object::new();

        if let Some(ranges) = ranges {
            Reflect::set(&options, &"includedRanges".into(), ranges).unwrap();
        }
        options.unchecked_into()
    }
}
