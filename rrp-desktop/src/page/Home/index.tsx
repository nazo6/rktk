import { commands, KeyActionLoc, KeyboardInfo } from "../../bindings";
import * as kle from "@ijprest/kle-serial";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { unwrapped } from "../../utils";
import { Keyboard } from "./Keyboard";

export type KeyData = {
  kleKey: kle.Key;
  actions: KeyActionLoc[];
};

export function Home({ keyboardInfo }: { keyboardInfo: KeyboardInfo }) {
  const queryClient = useQueryClient();

  const { data: layout } = useQuery({
    queryKey: ["getLayoutJson"],
    queryFn: async () => {
      const layoutJson = await unwrapped(commands.getLayoutJson)();
      const layout = kle.Serial.parse(
        JSON.stringify(JSON.parse(layoutJson).keymap),
      );
      return layout;
    },
  });

  const { data: keys, dataUpdatedAt } = useQuery({
    enabled: !!layout,
    queryKey: ["getKeymaps"],
    queryFn: async () => {
      const keymaps = await unwrapped(commands.getKeymaps)();
      const keys: KeyData[] = layout!.keys.map((key) => {
        return {
          kleKey: key,
          actions: Array.from(Array(keyboardInfo.layers)).map((_, layer) => {
            const col = parseInt(key.labels[0].split(",")[1]);
            const row = parseInt(key.labels[0].split(",")[0]);
            return keymaps.find((k) => {
              return k.col == col &&
                k.row == row &&
                k.layer == layer;
            }) ?? { col, row, layer, key: "Inherit" };
          }),
        };
      });

      return keys;
    },
  });

  const setKey = useMutation({
    mutationFn: async (changes: KeyActionLoc[]) => {
      await unwrapped(commands.setKeymaps)(changes);
    },
  });

  return (
    <div className="p-3">
      {keys
        ? (
          <Keyboard
            key={dataUpdatedAt}
            keys={keys}
            layers={keyboardInfo.layers}
            updateKeymap={async (changes) => {
              await setKey.mutateAsync(changes);
              queryClient.invalidateQueries({ queryKey: ["getKeymaps"] });
            }}
          />
        )
        : <div>Loading layout...</div>}
    </div>
  );
}
