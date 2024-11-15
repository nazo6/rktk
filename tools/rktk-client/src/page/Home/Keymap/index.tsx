import * as kle from "@ijprest/kle-serial";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Keyboard } from "./Keyboard";
import { KeyData, KeyLoc, KeyUpdate } from "./types";
import { KeySelector } from "./KeySelector";
import { useState } from "react";
import { deepEqual } from "fast-equals";
import { produce } from "immer";
import {
  Toast,
  ToastTitle,
  useToastController,
} from "@fluentui/react-components";
import { Toolbar } from "./Toolbar";
import { Connection } from "@/lib/connection";
import { KeyActionLoc } from "rktk-rrp-client-webhid";

export function KeymapPage({ connection }: { connection: Connection }) {
  const { data: layout, error: layoutError } = useQuery({
    queryKey: ["getLayoutJson"],
    queryFn: async () => {
      const layoutJson = await connection.client.get_layout_json();
      const layout = kle.Serial.parse(
        JSON.stringify(JSON.parse(layoutJson).keymap),
      );
      return layout;
    },
  });

  const { data: keys, dataUpdatedAt, error: keymapError } = useQuery({
    enabled: !!layout,
    queryKey: ["getKeymaps"],
    queryFn: async () => {
      const keymaps = await connection.client.get_keymaps();
      const keys: KeyData[] = [];
      layout?.keys.forEach((key) => {
        const col = parseInt(key.labels[0].split(",")[1]);
        const row = parseInt(key.labels[0].split(",")[0]);

        keymaps.forEach((k) => {
          if (k.col == col && k.row == row) {
            keys.push({
              kleKey: key,
              changed: false,
              action: k.key,
              loc: { col, row, layer: k.layer },
            });
          }
        });
      });

      return keys;
    },
  });

  return keys
    ? (
      <HomeInner
        key={dataUpdatedAt}
        connection={connection}
        keyDataUpdatedAt={dataUpdatedAt}
        keyData={keys}
      />
    )
    : (
      <div className="flex justify-center items-center pt-5">
        {layoutError && <div>Error fetching layout: {layoutError.message}</div>}
        {keymapError && <div>Error fetching keymap: {keymapError.message}</div>}
      </div>
    );
}

function HomeInner({ connection, keyData }: {
  connection: Connection;
  keyData: KeyData[];
  keyDataUpdatedAt: number;
}) {
  const queryClient = useQueryClient();
  const { dispatchToast } = useToastController();

  const [modifiedKeysData, setModifiedKeysData] = useState<KeyData[]>(keyData);
  const toUpdateKeyActions = modifiedKeysData.reduce<
    { crr: KeyUpdate; new: KeyUpdate }[]
  >((prev, crr, i) => {
    if (crr.changed) {
      prev.push({ crr: keyData[i], new: crr });
    }
    return prev;
  }, []);

  const [layer, setLayer] = useState(0);
  const [selectedKeyLoc, setSelectedKeyLoc] = useState<KeyLoc | null>(null);
  const selectedKeyIdx = modifiedKeysData.findIndex((kd) =>
    deepEqual(kd.loc, selectedKeyLoc)
  );
  const selectedKey = selectedKeyIdx >= 0
    ? modifiedKeysData[selectedKeyIdx]
    : null;

  const setKey = useMutation({
    mutationFn: async (changes: KeyActionLoc[]) => {
      await connection.client.set_keymaps(changes);
    },
    onSuccess: (_, changes) => {
      queryClient.invalidateQueries({ queryKey: ["getKeymaps"] });
      dispatchToast(
        <Toast>
          <ToastTitle>
            Updated {changes.length} key(s)
          </ToastTitle>
        </Toast>,
        { intent: "success" },
      );
    },
  });

  return (
    <div className="flex flex-col items-center">
      <Toolbar
        toUpdateKeyActions={toUpdateKeyActions}
        updateKeymap={async (updates) => {
          await setKey.mutateAsync(updates.map((update) => ({
            row: update.loc.row,
            col: update.loc.col,
            layer: update.loc.layer,
            key: update.action,
          })));
        }}
        clearKeymapModifications={() => {
          setModifiedKeysData(keyData);
        }}
      />
      <div className="w-full p-4">
        <Keyboard
          keys={modifiedKeysData}
          layer={layer}
          layerCount={connection.keyboard.keymap.layer_count}
          setLayer={setLayer}
          selectKeyLoc={(key) => setSelectedKeyLoc(key)}
          selectedKeyLoc={selectedKeyLoc}
        />
      </div>
      <KeySelector
        layerCount={connection.keyboard.keymap.layer_count}
        restoreSelectedKey={() => {
          setModifiedKeysData(produce(modifiedKeysData, (draft) => {
            draft[selectedKeyIdx] = keyData[selectedKeyIdx];
          }));
        }}
        selectedKey={selectedKey}
        onChange={(action) => {
          const newKeyData = { ...selectedKey!, action, changed: true };

          if (deepEqual(keyData[selectedKeyIdx].action, action)) {
            newKeyData.changed = false;
          }

          setModifiedKeysData(produce(modifiedKeysData, (draft) => {
            draft[selectedKeyIdx] = newKeyData;
          }));
        }}
      />
    </div>
  );
}
