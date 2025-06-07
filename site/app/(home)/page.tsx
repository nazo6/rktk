import { Callout } from "fumadocs-ui/components/callout";
import Link from "next/link";
import { AiFillGithub } from "react-icons/ai";
import { BiSolidRightArrowAlt } from "react-icons/bi";

export default function HomePage() {
  return (
    <main className="flex flex-1 flex-col items-center text-center">
      <div className="max-w-[1200px] w-full flex flex-col">
        <Callout className="mx-3">This document is in construction</Callout>
        <div className="flex flex-col items-center">
          <h1 className="max-w-2xl mb-4 text-4xl font-bold tracking-tight leading-none md:text-5xl xl:text-6xl dark:text-white">
            RKTK
          </h1>
          <p className="max-w-2xl mb-6 font-light text-gray-500 lg:mb-8 text-2xl dark:text-gray-400">
            Rust keyboard firmware framework
          </p>
          <div className="flex gap-3">
            <Link
              href="/docs"
              className="inline-flex items-center justify-center px-5 py-3 border-2 border-fd-primary rounded-full hover:bg-fd-accent"
            >
              Get started
              <BiSolidRightArrowAlt size="25px" />
            </Link>
            <Link
              href="https://github.com/nazo6/rktk"
              target="_blank"
              className="inline-flex items-center justify-center px-5 py-3 border-2 border-fd-secondary rounded-full hover:bg-fd-accent"
            >
              View on Github
              <AiFillGithub className="ml-1" size="25px" />
            </Link>
          </div>
        </div>
      </div>
    </main>
  );
}
