import { defineConfig, defineDocs } from "fumadocs-mdx/config";
import remarkDirective from "remark-directive";
import { remarkDirectiveDriverTable } from "./lib/remarkDirectiveDriverTable";
import mdxMermaid from "mdx-mermaid";

export const docs = defineDocs({
  dir: "content/docs",
});

export default defineConfig({
  mdxOptions: {
    remarkPlugins: (
      v,
    ) => [
      [mdxMermaid, { output: "svg" }],
      remarkDirective,
      remarkDirectiveDriverTable,
      ...v,
    ],
  },
  lastModifiedTime: "git",
});
