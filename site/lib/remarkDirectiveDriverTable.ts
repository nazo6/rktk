import { Root, Table, TableRow } from "mdast";
import { visit } from "unist-util-visit";

export function remarkDirectiveDriverTable() {
  return function (tree: Root) {
    visit(tree, (node, index, parent) => {
      if (
        node.type === "containerDirective" &&
        node.name === "drivers_table"
      ) {
        const child = node.children[0];
        if (child.type != "table") {
          throw new Error("Expected a table node");
        }
        const rows = child.children.slice(1);

        const newTable: Table = {
          type: "table",
          align: ["left", "left", "left"],
          children: [
            {
              type: "tableRow",
              children: [
                {
                  type: "tableCell",
                  children: [{ type: "text", value: "name" }],
                },
                {
                  type: "tableCell",
                  children: [{ type: "text", value: "url" }],
                },
                {
                  type: "tableCell",
                  children: [{ type: "text", value: "description" }],
                },
              ],
            },
            ...rows.map((row) => {
              const cells = row.children;
              if (cells.length != 4) {
                throw new Error("Expected 4 cells");
              }
              const [name, crate, path, description] = cells.slice(0, 4);
              const crateName: string = (crate.children[0] as any).value;
              const pathName: string = (path.children[0] as any).value;
              return {
                type: "tableRow",
                children: [
                  name,
                  {
                    type: "tableCell",
                    children: [{
                      type: "link",
                      url: `https://rktk-docs.nazo6.dev/${
                        crateName.replaceAll("-", "_")
                      }/${pathName}/index.html`,
                      children: [{
                        type: "text",
                        value: "docs",
                      }],
                    }],
                  },
                  description,
                ],
              } satisfies TableRow;
            }),
          ],
        };
        parent!.children[index!] = newTable as any;
      }
    });
  };
}
