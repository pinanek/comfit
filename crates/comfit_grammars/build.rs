//! Compile feature-selected grammars and generate their Rust bindings.

use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
};

use quote::{format_ident, quote};
use toml::Value;

fn main() {
    // SAFETY: Cargo supplies the jobserver descriptors to this build script.
    let jobserver = unsafe { jobserver::Client::from_env() };
    println!("cargo:rerun-if-changed=data/languages.toml");
    println!("cargo:rerun-if-env-changed=COMFIT_CHECK_ONLY");

    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("languages.rs");
    let has_languages = env::vars_os().any(|(name, _)| {
        name.to_str().is_some_and(|name| {
            name.starts_with("CARGO_FEATURE_") && name != "CARGO_FEATURE_DEFAULT"
        })
    });

    if !has_languages {
        let file = syn::parse_quote! {
        /// Language definitions selected by enabled Cargo features.
        pub const LANGUAGES: &[Language] = &[];
        /// File extensions (without a dot) paired with enabled language IDs.
        pub const FILE_EXTENSIONS: &[(&str, &str)] = &[];
        };
        fs::write(output, prettyplease::unparse(&file)).expect("writing empty grammar bindings");

        return;
    }

    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let metadata = fs::read_to_string(root.join("data/languages.toml")).expect(
        "missing grammar data; run `cargo grammars update` and `cargo grammars build` first",
    );
    let registry: Value = toml::from_str(&metadata).expect("invalid generated grammar data");
    let languages = registry["languages"]
        .as_table()
        .expect("missing generated language metadata");

    let enabled: Vec<_> = languages
        .iter()
        .filter(|(id, _)| {
            let feature = format!(
                "CARGO_FEATURE_{}",
                id.replace('-', "_").to_ascii_uppercase()
            );

            env::var_os(feature).is_some()
        })
        .collect();
    let grammars: BTreeSet<_> = enabled
        .iter()
        .map(|(_, language)| language["grammar"].as_str().expect("missing grammar ID"))
        .collect();

    // Metadata-only checks need the Rust bindings, not linked parser archives.
    if !env::var("COMFIT_CHECK_ONLY").is_ok_and(|value| value == "1") {
        let grammars: Vec<_> = grammars.into_iter().collect();
        let next = AtomicUsize::new(0);
        let compile = || {
            while let Some(id) = grammars.get(next.fetch_add(1, Ordering::Relaxed)) {
                compile_grammar(&root, &registry["grammars"][*id], id);
            }
        };
        let (sender, receiver) = mpsc::channel();
        let helper = jobserver
            .filter(|_| {
                grammars.len() > 1
                    && thread::available_parallelism().is_ok_and(|count| count.get() > 1)
            })
            .map(|client| {
                client
                    .into_helper_thread(move |token| {
                        let _ = sender.send(token);
                    })
                    .expect("starting Cargo jobserver helper")
            });
        if let Some(helper) = &helper {
            helper.request_token();
        }
        thread::scope(|scope| {
            if helper.is_some() {
                let compile = &compile;
                scope.spawn(move || {
                    if let Ok(token) = receiver.recv() {
                        let _token = token.expect("acquiring Cargo jobserver token");
                        compile();
                    }
                });
            }
            compile();
            // Cancel a pending acquisition before joining the worker, including with --jobs 1.
            drop(helper);
        });
    }

    let mut bindings = quote! {};
    let mut definitions = Vec::new();
    let mut extensions = Vec::new();

    for (id, language) in enabled {
        let grammar = language["grammar"].as_str().unwrap();
        let symbol = registry["grammars"][grammar]["symbol"].as_str().unwrap();
        let symbol = format_ident!("{symbol}");
        let module = format_ident!("{}", id.replace('-', "_"));
        let name = language["name"].as_str().unwrap();
        let aliases: Vec<_> = language["aliases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|alias| alias.as_str().unwrap())
            .collect();
        for extension in language["extensions"].as_array().unwrap() {
            let extension = extension.as_str().unwrap();
            extensions.push(quote! { (#extension, #id) });
        }

        definitions.push(quote! {
            Language {
                id: #id, name: #name, aliases: &[#(#aliases),*],
                language: #module::LANGUAGE,
                highlights: #module::HIGHLIGHTS_QUERY,
                injections: #module::INJECTIONS_QUERY,
                locals: #module::LOCALS_QUERY,
                rainbows: #module::RAINBOWS_QUERY,
            }
        });

        let queries = root.join("data/queries").join(id);
        println!("cargo:rerun-if-changed={}", queries.display());

        let mut query_bindings = Vec::new();

        for (name, kind) in [
            ("HIGHLIGHTS_QUERY", "highlights"),
            ("INJECTIONS_QUERY", "injections"),
            ("LOCALS_QUERY", "locals"),
            ("RAINBOWS_QUERY", "rainbows"),
        ] {
            let path = queries.join(format!("{kind}.scm"));
            assert!(
                path.is_file(),
                "missing {id}/{kind}.scm; run `cargo grammars update` first"
            );

            let name = format_ident!("{name}");
            let path = path.to_str().expect("query path is not UTF-8");

            let test = format_ident!("{kind}_query");
            query_bindings.push(quote! {
                #[doc = concat!("The ", #kind, " query, including inherited queries.")]
                pub const #name: &str = include_str!(#path);

                #[cfg(test)]
                #[test]
                fn #test() {
                    use streaming_iterator::StreamingIterator;

                    let language: tree_sitter::Language = LANGUAGE.into();
                    let query = tree_sitter::Query::new(&language, #name)
                        .expect("query must compile against its bundled grammar");
                    let mut parser = tree_sitter::Parser::new();
                    parser.set_language(&language).expect("parser must accept its bundled grammar");
                    let mut cursor = tree_sitter::QueryCursor::new();

                    // Query compatibility does not require syntactically valid source.
                    for source in ["", "\n", "name = \"café 😀\";\ncall([1, 2]);\n"] {
                        let tree = parser.parse(source, None).expect("parser must return a tree");
                        let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
                        while let Some(matched) = matches.next() {
                            for capture in matched.captures() {
                                assert!(
                                    source.get(capture.node.byte_range()).is_some(),
                                    "capture must cover a valid UTF-8 source range",
                                );
                            }
                        }
                    }
                }
            });
        }

        let parser = quote! {
            use tree_sitter_language::LanguageFn;

            unsafe extern "C" {
                fn #symbol() -> *const ();
            }

            // SAFETY: This is a language entry point generated by Tree-sitter.
            /// Tree-sitter parser entry point.
            pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(#symbol) };
        };

        bindings.extend(quote! {
            #[doc = concat!("Parser and Comfit queries for ", #name, ".")]
            pub mod #module {
                #parser
                #(#query_bindings)*
            }
        });
    }

    bindings.extend(quote! {
    /// Language definitions selected by enabled Cargo features.
    pub const LANGUAGES: &[Language] = &[#(#definitions),*];
    /// File extensions (without a dot) paired with enabled language IDs.
    pub const FILE_EXTENSIONS: &[(&str, &str)] = &[#(#extensions),*];
    });

    let file = syn::parse2(bindings).expect("invalid generated language modules");

    fs::write(output, prettyplease::unparse(&file)).expect("writing language modules");
}

fn compile_grammar(root: &Path, grammar: &Value, id: &str) {
    let path = grammar["path"].as_str().expect("missing grammar path");
    let directory = root.join("data/grammars").join(id);
    let generated = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join(id);
    println!("cargo:rerun-if-changed={}", directory.display());
    copy(&directory, &generated);
    let source = generated.join(path).join("src");

    let parser = source.join("parser.c");
    let prebuilt = grammar
        .get("prebuilt")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if prebuilt {
        if !directory.join(path).join("src/parser.c").is_file() {
            let input = fs::File::open(source.join("parser.c.zst"))
                .expect("missing prebuilt parser; run `cargo grammars build` first");
            let mut decoder =
                zstd::stream::read::Decoder::new(input).expect("opening compressed parser");
            let mut output = fs::File::create(&parser).expect("creating parser source");
            std::io::copy(&mut decoder, &mut output).expect("decompressing parser");
        }
    } else {
        let input = fs::read(source.join("grammar.json"))
            .expect("missing grammar definition; run `cargo grammars update` first");
        let json = std::str::from_utf8(&input).expect("invalid grammar definition UTF-8");
        let (_, code) = tree_sitter_generate::generate_parser_for_grammar(
            json,
            None,
            tree_sitter_generate::OptLevel::default(),
            &mut Vec::new(),
        )
        .unwrap_or_else(|error| panic!("generating parser for {id}: {error}"));
        fs::write(&parser, code).expect("writing generated parser");
    }

    if !prebuilt {
        let headers = source.join("tree_sitter");
        fs::create_dir_all(&headers).expect("creating generated header directory");
        for (name, contents) in [
            ("parser.h", tree_sitter_generate::PARSER_HEADER),
            ("alloc.h", tree_sitter_generate::ALLOC_HEADER),
            ("array.h", tree_sitter_generate::ARRAY_HEADER),
        ] {
            fs::write(headers.join(name), contents).expect("writing generated header");
        }
    }

    let mut build = cc::Build::new();
    build.include(&source).std("c11").warnings(false);

    if source.join("scanner.c").is_file() {
        // Keep cc from acquiring nested jobserver tokens inside our grammar workers.
        let objects = build
            .clone()
            .file(source.join("scanner.c"))
            .compile_intermediates();
        build.objects(objects);
    }

    build
        .file(source.join("parser.c"))
        .compile(&format!("comfit_{id}"));

    for scanner in ["scanner.cc", "scanner.cpp"] {
        if source.join(scanner).is_file() {
            cc::Build::new()
                .cpp(true)
                .include(&source)
                .file(source.join(scanner))
                .std("c++17")
                .warnings(false)
                .compile(&format!("comfit_{id}_scanner"));
        }
    }
}

fn copy(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("creating native grammar directory");
    for entry in fs::read_dir(source).expect("reading native grammar sources") {
        let entry = entry.expect("reading native grammar entry");
        let output = destination.join(entry.file_name());
        if entry.path().is_dir() {
            copy(&entry.path(), &output);
        } else {
            fs::copy(entry.path(), output).expect("copying native grammar source");
        }
    }
}
