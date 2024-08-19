import { notifications } from "@mantine/notifications";
import {
  commands,
  KeyAction,
  KeyActionLoc,
  KeyboardInfo,
  Result,
} from "../../bindings";
import { useEffect, useState } from "react";
import { Button } from "@mantine/core";

export function Home() {
  const [data, setData] = useState<KeyboardInfo | null>(null);

  useEffect(() => {
    (async () => {
      const data = await Promise.race([
        new Promise<Result<KeyboardInfo, string>>((resolve) =>
          commands.getKeyboardInfo().then((res) => resolve(res))
        ),
        new Promise<"timeout">((resolve) =>
          setTimeout(() => resolve("timeout"), 1000)
        ),
      ]);
      if (data == "timeout") {
        notifications.show({
          message: "Failed to connect to keyboard (timeout). Disconnecting...",
        });
        await commands.disconnect();
      } else {
        if (data.status == "ok") {
          setData(data.data);
        } else {
          notifications.show({
            message:
              `Failed to connect to keyboard (${data.error}). Disconnecting...`,
          });
          await commands.disconnect();
        }
      }
    })();
  }, []);

  return (
    data ? <HomeInner keyboardInfo={data} /> : <div>Connecting...</div>
  );
}

function HomeInner(props: { keyboardInfo: KeyboardInfo }) {
  const [keymaps, setKeymaps] = useState<KeyActionLoc[] | null>(null);
  const [layout, setLayout] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      let res = await commands.getKeymaps();
      if (res.status == "ok") {
        setKeymaps(res.data);
      } else {
        notifications.show({
          message: `Failed to get keymaps (${res.error})`,
        });
      }
    })();
  }, []);

  useEffect(() => {
    (async () => {
      let res = await commands.getLayoutJson();
      if (res.status == "ok") {
        setLayout(res.data);
      } else {
        notifications.show({
          message: `Failed to get layout (${res.error})`,
        });
      }
    })();
  }, []);

  return (
    <div>
      <h1>Connected to {props.keyboardInfo.name}</h1>
      <Button
        onClick={async () => {
          await commands.disconnect();
        }}
      >
        Disconnect
      </Button>
      {layout && keymaps
        ? <Keyboard keyboardJson={layout} keymaps={keymaps} />
        : <div>Loading layout...</div>}
    </div>
  );
}

import * as kle from "@ijprest/kle-serial";
import { KeyActionSelector } from "./KeySelector";

type KeyWithAction = {
  key: kle.Key;
  action?: KeyActionLoc;
};

function Keyboard(props: { keymaps: KeyActionLoc[]; keyboardJson: string }) {
  const val = JSON.parse(props.keyboardJson);
  const kb = kle.Serial.parse(JSON.stringify(val.keymap));
  const keys = kb.keys.map((key) => {
    return {
      key,
      action: props.keymaps.find((v) =>
        v.col == parseInt(key.labels[0].split(",")[1]) &&
        v.row == parseInt(key.labels[0].split(",")[0]) &&
        v.layer == 0
      ),
    };
  });

  const [selectedKey, setSelectedKey] = useState<KeyWithAction | null>(null);

  return (
    <div className="flex flex-col">
      <div className="relative h-72">
        {keys.map((k) => (
          <Key
            k={k}
            onClick={() => {
              setSelectedKey(k);
            }}
          />
        ))}
      </div>
      <div>
        {selectedKey
          ? (
            <div className="flex gap-2">
              <p>Row: {selectedKey.action?.row}</p>
              <p>Col: {selectedKey.action?.col}</p>
              <p>Layer: {selectedKey.action?.layer}</p>
            </div>
          )
          : <div>No key selected</div>}
      </div>
      {selectedKey?.action &&
        (
          <KeyActionSelector
            keyAction={selectedKey.action.key}
            setKeyAction={(key) => {}}
          />
        )}
    </div>
  );
}

function Key(
  { k, onClick }: {
    k: KeyWithAction;
    onClick?: () => void;
  },
) {
  return (
    <div
      className="absolute border-2 border-black"
      style={{
        width: k.key.width * 50 + "px",
        height: k.key.height * 50 + "px",
        top: k.key.y * 50 + "px",
        left: k.key.x * 50 + "px",
        transform: `rotate(${k.key.rotation_angle}deg)`,
      }}
      onClick={onClick}
    >
      {displayKey(k.action?.key)}
    </div>
  );
}

function displayKey(key?: KeyAction): string {
  if (!key) {
    return "";
  }
  if (typeof key != "string" && "Normal" in key) {
    if (typeof key.Normal != "string" && "Key" in key.Normal) {
      return key.Normal.Key;
    }
  }

  return "";
}
