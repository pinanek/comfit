# Documentation site

The site uses Waku, React, and Markdown compiled through MDX. Run commands from the repository root unless noted otherwise.

## Source layout

```text
content/                  Guides and generated API references
plugins/                  Build-time Markdown transforms
public/                   Static assets
src/
  pages/                  Waku routes, layouts, and route configuration
  features/
    documentation/        Document discovery, links, navigation, and reading UI
    search/               Search dialog and Pagefind loading
    playground/           Editor, WASM worker, language samples, and themes
    home/                 Homepage and its interactive examples
  components/             Shared controls and site navigation
  hooks/                  Shared browser hooks
  lib/                    Shared utilities
  mdx/                    Components used to render Markdown
  styles/                 Global styles and design tokens
```

Keep feature-specific helpers and CSS beside their components. Route files delegate to features. Markdown transforms belong in `plugins/`, not in runtime components.

`features/documentation/documents.ts` owns document discovery, slug lookup, static paths, and Markdown link resolution. `document.tsx` renders the selected document. Handwritten navigation lives in `navigation.ts`, alongside the import of the generated API manifest.

The playground runs highlighting in a worker. It retains documents by language and skips superseded requests once grammar loading completes.

## Development

The docs site is part of the root pnpm workspace and uses its shared catalog and lockfile.

```sh
pnpm install --frozen-lockfile
pnpm --dir docs dev
pnpm --dir docs exec tsc --noEmit
pnpm exec oxlint -c docs/oxlint.config.ts docs
pnpm --dir docs build
```

The `dev` and `build` scripts build the local Node, rehype, WASM, and grammar packages first. The production build also generates the Pagefind search index. Search requires that generated index.

Use `just fmt` and `just lint` for workspace formatting and Rust/package checks.

## Generated files

- Run `cargo docs build` to regenerate `content/api-references/javascript/`, `content/api-references/python/`, and `content/api-references/manifest.json`. Do not edit generated reference pages directly.
- `content/api-references/rust.md` is authored content linking to docs.rs and is preserved during generation.
- Waku generates `src/pages.gen.ts`. Keep route files in `src/pages/` rather than editing the generated manifest.
- Build output belongs in `dist/`. WASM glue and grammar assets are generated in their respective packages.

Keep existing content paths and public URLs stable when reorganizing source code.

## Page metadata

`src/components/metadata.tsx` renders page titles, descriptions, canonical URLs, and Open Graph and Twitter tags. Defaults come from `src/site.ts`. Render it once per page, with the page's public `path` and any overrides.

Authored pages in `content/` export metadata from YAML frontmatter:

```yaml
---
title: HTML and CSS
description: Render highlighted code with inline styles, CSS classes, or CSS variables.
---
```

Frontmatter can also set `image`, `imageAlt`, and `type` (`website` or `article`). Titles receive the Comfit site-name suffix. The document route supplies `path` automatically.

Generated JavaScript and Python API pages use `content/api-references/manifest.json` instead of frontmatter. Run `cargo docs manifest` to refresh its `sidebar` and per-route `pages` metadata from existing reference files, or `cargo docs build` to regenerate all references.
