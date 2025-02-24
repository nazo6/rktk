import type { MetaRecord } from "nextra";

export default {
  index: {
    type: "page",
    display: "hidden",
    theme: {
      footer: false,
      timestamp: false,
      toc: false,
    },
  },
  docs: {
    type: "page",
    title: "Documentation",
  },
  link_client: {
    type: "page",
    title: "RKTK Client",
    href: "https://rktk-client.nazo6.dev",
  },
} satisfies MetaRecord;
