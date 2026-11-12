//! Download themes, resolve includes, and apply registry overrides.

use std::{
    borrow::Cow,
    collections::HashMap,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail, ensure};
use comfit_core::Theme;
use comfit_internal::theme::{Source, ThemeSource};
use comfit_themes::convert::{VscodeTheme, convert};
use reqwest::{Url, blocking::Client};
use zip::ZipArchive;

pub(crate) fn run(
    client: &Client,
    downloads: &mut HashMap<String, Vec<u8>>,
    source: &ThemeSource,
    debug: bool,
) -> Result<Theme> {
    if !downloads.contains_key(&source.url) {
        let bytes = client
            .get(&source.url)
            .send()?
            .error_for_status()?
            .bytes()?
            .to_vec();
        downloads.insert(source.url.clone(), bytes);
    }

    let bytes = &downloads[&source.url];
    let mut archive = match &source.source {
        Source::Json => None,
        Source::Extension { .. } => Some(ZipArchive::new(Cursor::new(bytes.as_slice()))?),
    };

    let location = match &source.source {
        Source::Json => source.url.as_str(),
        Source::Extension { path } => path.as_str(),
    };

    let raw = match archive.as_mut() {
        Some(archive) => Cow::Owned(read_entry(archive, location)?),
        None => Cow::Borrowed(std::str::from_utf8(bytes)?),
    };

    let mut parsed = VscodeTheme::default();
    load(client, &mut archive, location, &raw, &mut parsed, 0)?;
    // An explicit override replaces the upstream palette before color validation.
    if source.rainbow.is_some() {
        for color in [
            &mut parsed.colors.bracket_foreground_1,
            &mut parsed.colors.bracket_foreground_2,
            &mut parsed.colors.bracket_foreground_3,
            &mut parsed.colors.bracket_foreground_4,
            &mut parsed.colors.bracket_foreground_5,
            &mut parsed.colors.bracket_foreground_6,
        ] {
            *color = None;
        }
    }

    let conversion = convert(&source.name, &parsed)?;
    let mut theme = conversion.theme;

    if let Some(rainbow) = &source.rainbow {
        theme.rainbow = rainbow
            .iter()
            .map(|color| color.parse())
            .collect::<Result<_, _>>()?;
    }

    let skipped = conversion.unmapped_selectors;

    if debug && !skipped.is_empty() {
        let total: usize = skipped.values().sum();
        eprintln!(
            "{}: skipped {total} unmapped selectors ({} unique)",
            source.id,
            skipped.len()
        );

        for (selector, count) in skipped {
            eprintln!("  {count}x {selector:?}");
        }
    }

    Ok(theme)
}

fn read_entry(archive: &mut ZipArchive<Cursor<&[u8]>>, path: &str) -> Result<String> {
    let mut raw = String::new();
    archive
        .by_name(path)
        .with_context(|| format!("missing VSIX entry {path}"))?
        .read_to_string(&mut raw)?;

    Ok(raw)
}

fn load(
    client: &Client,
    archive: &mut Option<ZipArchive<Cursor<&[u8]>>>,
    location: &str,
    raw: &str,
    theme: &mut VscodeTheme,
    depth: usize,
) -> Result<()> {
    ensure!(depth < 16, "theme include nesting exceeds 16 at {location}");
    let mut parsed = VscodeTheme::from_json(raw).with_context(|| format!("parsing {location}"))?;

    if let Some(include) = parsed.include.take() {
        let (next, raw) = if let Some(zip) = archive.as_mut() {
            let parent = Path::new(location).parent().unwrap_or(Path::new(""));
            let mut normalized = PathBuf::new();

            for component in parent.join(include).components() {
                match component {
                    std::path::Component::Normal(part) => normalized.push(part),
                    std::path::Component::CurDir => {}
                    std::path::Component::ParentDir => {
                        ensure!(normalized.pop(), "include escapes archive");
                    }
                    _ => bail!("invalid archive include path"),
                }
            }

            let next = normalized.to_string_lossy().into_owned();
            let raw = read_entry(zip, &next)?;
            (next, raw)
        } else {
            let next = Url::parse(location)?.join(&include)?;
            let raw = client
                .get(next.clone())
                .send()?
                .error_for_status()?
                .text()?;
            (next.to_string(), raw)
        };

        load(client, archive, &next, &raw, theme, depth + 1)?;
    }

    theme.merge(parsed)?;

    Ok(())
}
