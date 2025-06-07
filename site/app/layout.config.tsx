import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";
import { AiFillGithub } from "react-icons/ai";
import { BiLinkExternal } from "react-icons/bi";

/**
 * Shared layout configurations
 *
 * you can customise layouts individually from:
 * Home Layout: app/(home)/layout.tsx
 * Docs Layout: app/docs/layout.tsx
 */
export const baseOptions: BaseLayoutProps = {
  nav: {
    title: (
      <div className="inline-flex justify-center items-center gap-2">
        <img
          alt="logo"
          src="/logo.webp"
          className="w-8 h-8 object-cover align-middle"
        />
        <p>RKTK</p>
      </div>
    ),
  },
  links: [
    {
      text: "Documentation",
      url: "/docs",
      active: "nested-url",
    },
    {
      text: (
        <p className="inline-flex items-center">
          API Docs
          <BiLinkExternal className="pl-1" />
        </p>
      ),
      url: "https://rktk-docs.nazo6.dev",
    },
    {
      text: (
        <p className="inline-flex items-center">
          RKTK Client
          <BiLinkExternal className="pl-1" />
        </p>
      ),
      url: "https://rktk-client.nazo6.dev",
    },
    {
      type: "icon",
      url: "https://github.com/nazo6/rktk",
      text: "Github",
      icon: <AiFillGithub />,
      external: true,
    },
  ],
};
