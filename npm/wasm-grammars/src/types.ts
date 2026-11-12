/**
 * Language descriptors and WASM byte sources shared by grammar packages.
 * @packageDocumentation
 */

/** A URL to fetch or WASM bytes. Importing a descriptor does not fetch its asset. */
export type WasmSource = string | URL | Uint8Array | ArrayBuffer;

/** A language's parser asset, ID, aliases, and Tree-sitter queries. */
export interface Language {
  /** Canonical language ID, unique within one highlighter. */
  readonly id: string;
  /** Additional names accepted by the highlighter. Aliases and IDs share one namespace. */
  readonly aliases?: readonly string[];
  /** Parser WASM URL or bytes. Languages can share the same parser asset. */
  readonly wasm: WasmSource;
  /** Token capture query. An empty string disables this pass. */
  readonly highlights: string;
  /** Embedded-language query. Omitted or empty queries disable this pass. */
  readonly injections?: string;
  /** Lexical scope, definition, and reference query. */
  readonly locals?: string;
  /** Query for brackets and nesting scopes, used to calculate bracket depths. */
  readonly rainbows?: string;
}
