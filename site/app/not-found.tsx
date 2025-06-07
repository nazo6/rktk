import { HomeLayout } from "fumadocs-ui/layouts/home";
import { baseOptions } from "@/app/layout.config";
import Link from "next/link";

export default function NotFound() {
  return (
    <HomeLayout {...baseOptions}>
      <main className="flex flex-1 flex-col justify-center text-center prose">
        <h2 className="mb-4 text-2xl font-bold">Not Found</h2>
        <p>Could not find requested resource</p>
        <Link href="/">Return Home</Link>
      </main>
    </HomeLayout>
  );
}
