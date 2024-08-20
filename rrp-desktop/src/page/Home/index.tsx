import { commands, KeyActionLoc, KeyboardInfo } from "../../bindings";
import * as kle from "@ijprest/kle-serial";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { unwrapped } from "../../utils";
import { Keyboard } from "./Keyboard";
import { KeyData, KeyLoc, KeyUpdate } from "./types";
import { KeySelector } from "./KeySelector";
import { useState } from "react";
import { deepEqual } from "fast-equals";
import { produce } from "immer";
import { KeyUpdater } from "./KeyUpdate";
import {
  Toast,
  ToastTitle,
  useToastController,
} from "@fluentui/react-components";

export function Home({ keyboardInfo }: { keyboardInfo: KeyboardInfo }) {
  const queryClient = useQueryClient();
  const { dispatchToast } = useToastController();

  const { data: layout, error: layoutError } = useQuery({
    queryKey: ["getLayoutJson"],
    queryFn: async () => {
      const layoutJson = await unwrapped(commands.getLayoutJson)();
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
      const keymaps = await unwrapped(commands.getKeymaps)();
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

  const setKey = useMutation({
    mutationFn: async (changes: KeyActionLoc[]) => {
      await unwrapped(commands.setKeymaps)(changes);
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
    <div>
      {keys
        ? (
          <HomeInner
            keyboardInfo={keyboardInfo}
            keyDataUpdatedAt={dataUpdatedAt}
            keyData={keys}
            updateKeymap={(updates) =>
              setKey.mutateAsync(updates.map((update) => ({
                row: update.loc.row,
                col: update.loc.col,
                layer: update.loc.layer,
                key: update.action,
              })))}
          />
        )
        : (
          <div className="flex justify-center items-center pt-5">
            {layoutError && (
              <div>Error fetching layout: {layoutError.message}</div>
            )}
            {keymapError && (
              <div>Error fetching keymap: {keymapError.message}</div>
            )}
          </div>
        )}
    </div>
  );
}

function HomeInner({ keyboardInfo, keyData, updateKeymap }: {
  keyboardInfo: KeyboardInfo;
  keyData: KeyData[];
  keyDataUpdatedAt: number;
  updateKeymap: (changes: KeyUpdate[]) => Promise<void>;
}) {
  const [modifiedKeysData, setModifiedKeysData] = useState<KeyData[]>(keyData);
  const [toUpdateKeyActions, setToUpdateKeyActions] = useState<
    { current: KeyUpdate; new: KeyUpdate }[]
  >([]);

  const [layer, setLayer] = useState(0);
  const [selectedKeyLoc, setSelectedKeyLoc] = useState<KeyLoc | null>(null);
  const selectedKey = modifiedKeysData.find((kd) =>
    deepEqual(kd.loc, selectedKeyLoc)
  );

  return (
    <div className="flex flex-col items-center">
      <Keyboard
        keys={modifiedKeysData}
        layer={layer}
        layerCount={keyboardInfo.layers}
        setLayer={setLayer}
        selectKeyLoc={(key) => setSelectedKeyLoc(key)}
        selectedKeyLoc={selectedKeyLoc}
      />
      <div className="p-2">
        <KeyUpdater
          toUpdateKeyActions={toUpdateKeyActions}
          updateKeymap={updateKeymap}
          clearKeymapModifications={() => {
            setToUpdateKeyActions([]);
            setModifiedKeysData(keyData);
          }}
        />
      </div>
      <div className="p-2">
        <KeySelector
          selectedKey={selectedKey}
          onChange={(action) => {
            const keyIdx = modifiedKeysData.findIndex((kd) =>
              deepEqual(kd.loc, selectedKeyLoc)
            );
            const currentKeyData = modifiedKeysData[keyIdx];
            const newKeyData = { ...currentKeyData, action, changed: true };

            setToUpdateKeyActions([
              ...toUpdateKeyActions,
              { current: currentKeyData, new: newKeyData },
            ]);
            setModifiedKeysData(produce(modifiedKeysData, (draft) => {
              draft[keyIdx] = newKeyData;
            }));
          }}
        />
      </div>
    </div>
  );
}
