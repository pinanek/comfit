//! Compile bundled TOML themes and generate their Rust initialization functions.

use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result};
use comfit_core::{Color, Modifier, Style, Theme, UnderlineStyle};
use quote::{ToTokens, format_ident, quote};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").context("missing crate directory")?);
    let output = PathBuf::from(env::var_os("OUT_DIR").context("missing build output directory")?)
        .join("themes.rs");
    let mut enabled: Vec<_> = env::vars_os()
        .filter_map(|(name, _)| {
            let name = name.to_str()?.strip_prefix("CARGO_FEATURE_")?;

            (!matches!(name, "DEFAULT" | "ALL_THEMES" | "CONVERT"))
                .then(|| name.to_ascii_lowercase().replace('_', "-"))
        })
        .collect();
    enabled.sort();

    let mut modules = quote! {};
    let mut definitions = Vec::with_capacity(enabled.len());

    for id in enabled {
        let path = root.join("data").join(format!("{id}.toml"));
        println!("cargo:rerun-if-changed={}", path.display());
        let source = fs::read_to_string(&path)
            .with_context(|| format!("reading theme {}", path.display()))?;
        let theme = Theme::from_toml(&source)
            .with_context(|| format!("parsing theme {}", path.display()))?;
        let name = &theme.name;
        let module = format_ident!("{}", id.replace('-', "_"));
        let default = style_tokens(&theme.default);
        let line_highlight = style_tokens(&theme.line_highlight);
        let rainbow = theme.rainbow.iter().copied().map(color_tokens);
        let styles = theme.styles.iter().map(|(capture, style)| {
            let style = style_tokens(style);
            quote! { (String::from(#capture), #style) }
        });

        modules.extend(quote! {
            #[doc = concat!("Bundled ", #name, " theme.")]
            pub mod #module {
                static THEME: std::sync::LazyLock<comfit_core::Theme> =
                    std::sync::LazyLock::new(|| comfit_core::Theme {
                        name: String::from(#name),
                        default: #default,
                        styles: std::collections::BTreeMap::from([#(#styles),*]),
                        rainbow: vec![#(#rainbow),*],
                        line_highlight: #line_highlight,
                    });

                /// Return the shared theme, initializing it on first access.
                ///
                /// Concurrent callers share one initialization. Clone the returned
                /// theme if you need to customize it without changing the shared value.
                pub fn theme() -> &'static comfit_core::Theme {
                    &THEME
                }
            }
        });
        definitions.push(quote! {
            ThemeDefinition { id: #id, name: #name, theme: #module::theme }
        });
    }

    let generated = quote! {
        #modules

        /// Enabled theme definitions in theme-ID order.
        ///
        /// Reading the catalog does not initialize any themes. With no theme
        /// features enabled, this catalog is empty.
        pub const THEMES: &[ThemeDefinition] = &[#(#definitions),*];
    };
    let file = syn::parse2(generated).context("invalid generated theme code")?;
    fs::write(output, prettyplease::unparse(&file)).context("writing generated theme code")?;

    Ok(())
}

fn style_tokens(style: &Style) -> impl ToTokens {
    let fg = option_tokens(style.fg.map(color_tokens));
    let bg = option_tokens(style.bg.map(color_tokens));
    let modifiers = style.modifiers.iter().copied().map(modifier_tokens);
    let underline = option_tokens(style.underline.as_ref().map(|underline| {
        let color = option_tokens(underline.color.map(color_tokens));
        let style = option_tokens(underline.style.map(underline_style_tokens));
        quote! { comfit_core::Underline { color: #color, style: #style } }
    }));

    quote! {
        comfit_core::Style {
            fg: #fg,
            bg: #bg,
            modifiers: vec![#(#modifiers),*],
            underline: #underline,
        }
    }
}

fn option_tokens(value: Option<impl ToTokens>) -> impl ToTokens {
    match value {
        Some(value) => quote! { Some(#value) },
        None => quote! { None },
    }
}

fn color_tokens(color: Color) -> impl ToTokens {
    match color {
        Color::Rgb(color) => {
            let (r, g, b) = (color.r, color.g, color.b);
            quote! { comfit_core::Color::Rgb(comfit_core::RGB8::new(#r, #g, #b)) }
        }
        Color::Indexed(index) => quote! { comfit_core::Color::Indexed(#index) },
        Color::Default => quote! { comfit_core::Color::Default },
    }
}

fn modifier_tokens(modifier: Modifier) -> impl ToTokens {
    match modifier {
        Modifier::Bold => quote! { comfit_core::Modifier::Bold },
        Modifier::Dim => quote! { comfit_core::Modifier::Dim },
        Modifier::Italic => quote! { comfit_core::Modifier::Italic },
        Modifier::SlowBlink => quote! { comfit_core::Modifier::SlowBlink },
        Modifier::RapidBlink => quote! { comfit_core::Modifier::RapidBlink },
        Modifier::Reversed => quote! { comfit_core::Modifier::Reversed },
        Modifier::Hidden => quote! { comfit_core::Modifier::Hidden },
        Modifier::CrossedOut => quote! { comfit_core::Modifier::CrossedOut },
    }
}

fn underline_style_tokens(style: UnderlineStyle) -> impl ToTokens {
    match style {
        UnderlineStyle::Line => quote! { comfit_core::UnderlineStyle::Line },
        UnderlineStyle::Curl => quote! { comfit_core::UnderlineStyle::Curl },
        UnderlineStyle::Dashed => quote! { comfit_core::UnderlineStyle::Dashed },
        UnderlineStyle::Dotted => quote! { comfit_core::UnderlineStyle::Dotted },
        UnderlineStyle::DoubleLine => quote! { comfit_core::UnderlineStyle::DoubleLine },
    }
}
