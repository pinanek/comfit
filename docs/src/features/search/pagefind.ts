export interface SearchResult {
  url: string;
  meta: { title: string };
  excerpt: string;
}

interface Pagefind {
  search(query: string): Promise<{ results: { data(): Promise<SearchResult & { plain_excerpt: string }> }[] }>;
}

let pagefind: Promise<Pagefind> | undefined;
let attempts = 0;

function loadSearch(): Promise<Pagefind> {
  if (!pagefind) {
    // Browsers cache failed module imports, so retries need a fresh URL.
    const url = `/pagefind/pagefind.js${attempts ? `?retry=${attempts}` : ""}`;
    pagefind = import(/* @vite-ignore */ url).catch((error: unknown) => {
      pagefind = undefined;
      attempts++;
      throw error;
    });
  }
  return pagefind;
}

export async function search(query: string): Promise<SearchResult[]> {
  const index = await loadSearch();
  const { results } = await index.search(query);
  const matches = await Promise.all(results.slice(0, 12).map((result) => result.data()));

  return matches.map(({ url, meta, plain_excerpt }) => ({ url, meta, excerpt: plain_excerpt }));
}
