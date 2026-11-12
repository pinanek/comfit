import rehypeComfit from "@comfit/rehype";
import mdx from "@mdx-js/rollup";
import babel from "@rolldown/plugin-babel";
import react, { reactCompilerPreset } from "@vitejs/plugin-react";
import { fontless } from "fontless";
import rehypeKatex from "rehype-katex";
import rehypeSlug from "rehype-slug";
import { defListHastHandlers, remarkDefinitionList } from "remark-definition-list";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import { defineConfig } from "waku/config";
import rehypeAlert from "./plugins/rehype-alert";
import rehypeCodeBlock from "./plugins/rehype-code-block";
import rehypeCodeTabs from "./plugins/rehype-code-tabs";
import rehypeFigure from "./plugins/rehype-figure";
import rehypeToc from "./plugins/rehype-toc";

export default defineConfig({
  vite: {
    worker: { format: "es" },
    environments: {
      rsc: {
        resolve: { external: ["takumi-js"] },
        optimizeDeps: { exclude: ["takumi-js"] },
      },
    },
    optimizeDeps: {
      exclude: ["lucide-react", "takumi-js"],
    },
    css: {
      transformer: "lightningcss",
      lightningcss: {
        drafts: {
          customMedia: true,
        },
      },
    },
    resolve: {
      tsconfigPaths: true,
    },
    plugins: [
      fontless({
        defaults: {
          styles: ["normal", "italic"],
        },
        families: [
          {
            name: "Hanken Grotesk",
            provider: "google",
            weights: ["100 900"],
            subsets: ["latin", "latin-ext", "vietnamese"],
          },
          {
            name: "JetBrains Mono",
            provider: "google",
            weights: ["100 800"],
            subsets: ["latin"],
          },
        ],
      }),
      mdx({
        include: /\.md$/,
        format: "mdx",
        mdxExtensions: [".md"],
        remarkPlugins: [
          remarkGfm,
          remarkFrontmatter,
          [remarkMdxFrontmatter, { name: "metadata" }],
          remarkMath,
          remarkDefinitionList,
        ],
        remarkRehypeOptions: { handlers: defListHastHandlers },
        rehypePlugins: [
          rehypeAlert,
          rehypeFigure,
          rehypeKatex,
          rehypeSlug,
          rehypeToc,
          [rehypeComfit, { classPrefix: "syntax-", rainbowColors: 6 }],
          rehypeCodeBlock,
          rehypeCodeTabs,
        ],
      }),
      react(),
      babel({ presets: [reactCompilerPreset()] }),
    ],
  },
});
