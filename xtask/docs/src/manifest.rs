use std::{collections::BTreeMap, fs, path::Path};

use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
enum Item {
    Link { label: String, href: String },
    Group { label: String, items: Vec<Item> },
}

#[derive(Serialize)]
struct Metadata {
    title: String,
    description: String,
}

#[derive(Serialize)]
struct Manifest {
    sidebar: Vec<Item>,
    pages: BTreeMap<String, Metadata>,
}

pub fn generate(root: &Path) -> Result<()> {
    let mut sidebar = Vec::new();
    let mut metadata = BTreeMap::new();

    for (directory, label) in [("javascript", "JavaScript"), ("python", "Python")] {
        let items = pages(
            &root.join(directory),
            &format!("/api-references/{directory}"),
            &mut metadata,
        )?;

        sidebar.push(Item::Group {
            label: label.to_owned(),
            items,
        });
    }

    let output = root.join("manifest.json");
    fs::write(
        &output,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&Manifest {
                sidebar,
                pages: metadata,
            })?
        ),
    )?;
    println!("Generated API manifest in {}", output.display());
    Ok(())
}

fn pages(
    directory: &Path,
    href: &str,
    metadata: &mut BTreeMap<String, Metadata>,
) -> Result<Vec<Item>> {
    let mut entries = fs::read_dir(directory)
        .with_context(|| format!("reading {}", directory.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|path| {
        let name = path.file_stem().unwrap_or_default().to_string_lossy();
        let order = match name.as_ref() {
            "enumerations" | "node" => 0,
            "classes" | "wasm" => 1,
            "interfaces" | "rehype" => 2,
            "type-aliases" | "wasm-grammars" => 3,
            "variables" => 4,
            "functions" => 5,
            _ => 6,
        };
        (order, name.into_owned())
    });

    let mut items = Vec::new();
    if directory.join("index.md").is_file() {
        add_metadata(&directory.join("index.md"), href, metadata)?;
        items.push(Item::Link {
            label: "Overview".to_owned(),
            href: href.to_owned(),
        });
    }

    for path in entries {
        let name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .context("API reference path is not UTF-8")?;
        let label = match name {
            "node" => "Node.js",
            "wasm" => "WebAssembly",
            "wasm-grammars" => "WASM grammars",
            "comfit_core" => "comfit-core",
            "comfit_grammars" => "comfit-grammars",
            "comfit_themes" => "comfit-themes",
            "enumerations" => "Enumerations",
            "classes" => "Classes",
            "interfaces" => "Interfaces",
            "type-aliases" => "Type aliases",
            "variables" => "Variables",
            "functions" => "Functions",
            _ => name,
        }
        .to_owned();
        let href = format!("{href}/{name}");

        if path.is_dir() {
            let mut children = pages(&path, &href, metadata)?;
            if path.with_extension("md").is_file() {
                children.insert(
                    0,
                    Item::Link {
                        label: "Overview".to_owned(),
                        href: href.clone(),
                    },
                );
            }
            if children.is_empty() {
                continue;
            }
            if path.join("index.md").is_file() && children.len() == 1 {
                items.push(Item::Link { label, href });
            } else {
                items.push(Item::Group {
                    label,
                    items: children,
                });
            }
        } else if path.extension().is_some_and(|extension| extension == "md") && name != "index" {
            add_metadata(&path, &href, metadata)?;
            if !path.with_extension("").is_dir() {
                items.push(Item::Link { label, href });
            }
        }
    }

    Ok(items)
}

fn add_metadata(path: &Path, href: &str, pages: &mut BTreeMap<String, Metadata>) -> Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let title = content
        .lines()
        .find_map(|line| {
            line.strip_prefix('#')
                .and_then(|heading| heading.trim_start_matches('#').strip_prefix(' '))
        })
        .with_context(|| format!("missing page title in {}", path.display()))?
        .replace('`', "");
    let language = if href.starts_with("/api-references/javascript/") {
        "JavaScript"
    } else {
        "Python"
    };
    pages.insert(
        href.to_owned(),
        Metadata {
            description: format!("{title}: Comfit {language} API reference."),
            title,
        },
    );
    Ok(())
}
