# @comfit/rehype

Comfit plugin for highlighting code blocks with rehype, backed by [@comfit/node](../node/README.md).

[Documentation](https://comfit.pinanek.me) · [npm](https://www.npmjs.com/package/@comfit/rehype)

## Installation

```sh
npm install @comfit/rehype unified remark-parse remark-rehype rehype-stringify
```

## Usage

````js
import rehypeComfit from "@comfit/rehype";
import rehypeStringify from "rehype-stringify";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";

const file = await unified()
  .use(remarkParse)
  .use(remarkRehype)
  .use(rehypeComfit, { theme: "catppuccin-mocha" })
  .use(rehypeStringify)
  .process("```rust {1}\nfn main() {}\n```\n");

console.log(String(file));
````

See the [Markdown and MDX guide](https://comfit.pinanek.me/docs/guides/markdown) for framework integration, fence metadata, themes, and configuration.

## License

[MIT](LICENSE). Native parser and query attribution is distributed with `@comfit/node`.
