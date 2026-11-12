// deno-fmt-ignore-file
// biome-ignore-all format: generated types do not need formatting
// prettier-ignore
import type { PathsForPages, GetConfigResponse, SearchCodecsForPages } from 'waku/router';

// prettier-ignore
import type { getConfig as File_ApiReferencesSlug_getConfig } from './pages/api-references/[...slug]';
// prettier-ignore
import type { getConfig as File_ApiReferencesIndex_getConfig } from './pages/api-references/index';
// prettier-ignore
import type { getConfig as File_DocsSlug_getConfig } from './pages/docs/[...slug]';
// prettier-ignore
import type { getConfig as File_DocsIndex_getConfig } from './pages/docs/index';
// prettier-ignore
import type { getConfig as File_Index_getConfig } from './pages/index';
// prettier-ignore
import type { getConfig as File_Logos_getConfig } from './pages/logos';
// prettier-ignore
import type { getConfig as File_Playground_getConfig } from './pages/playground';
// prettier-ignore
import type { getConfig as File_Layout_getConfig } from './pages/_layout';
// prettier-ignore
import type { getConfig as File_ApiReferencesLayout_getConfig } from './pages/api-references/_layout';
// prettier-ignore
import type { getConfig as File_DocsLayout_getConfig } from './pages/docs/_layout';

// prettier-ignore
type Page =
| ({ path: '/api-references/[...slug]' } & GetConfigResponse<typeof File_ApiReferencesSlug_getConfig>)
| ({ path: '/api-references' } & GetConfigResponse<typeof File_ApiReferencesIndex_getConfig>)
| ({ path: '/docs/[...slug]' } & GetConfigResponse<typeof File_DocsSlug_getConfig>)
| ({ path: '/docs' } & GetConfigResponse<typeof File_DocsIndex_getConfig>)
| ({ path: '/' } & GetConfigResponse<typeof File_Index_getConfig>)
| ({ path: '/logos' } & GetConfigResponse<typeof File_Logos_getConfig>)
| ({ path: '/playground' } & GetConfigResponse<typeof File_Playground_getConfig>);

// prettier-ignore
type Layout =
| ({ path: '/' } & Awaited<ReturnType<typeof File_Layout_getConfig>>)
| ({ path: '/api-references' } & Awaited<ReturnType<typeof File_ApiReferencesLayout_getConfig>>)
| ({ path: '/docs' } & Awaited<ReturnType<typeof File_DocsLayout_getConfig>>);

// prettier-ignore
declare module 'waku/router' {
  interface RouteConfig {
    paths: PathsForPages<Page>;
  }
  interface CreatePagesConfig {
    pages: Page;
    layouts: Layout;
  }
  interface SearchCodecsConfig extends SearchCodecsForPages<Page> {}
}
