import { notifications } from "@mantine/notifications";
import {
  commands,
  KeyAction,
  KeyboardInfo,
  KeyDef,
  KeyDefLoc,
  Result,
} from "../../bindings";
import { useEffect, useState } from "react";
import { Button, NavLink } from "@mantine/core";

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
  const [keymaps, setKeymaps] = useState<KeyDefLoc[] | null>(null);
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

function Keyboard(props: { keymaps: KeyDefLoc[]; keyboardJson: string }) {
  console.log(props.keyboardJson);

  const val = JSON.parse(props.keyboardJson);
  const kb = kle.Serial.parse(JSON.stringify(val.keymap));

  console.log(kb);

  const [selectedKey, setSelectedKey] = useState<
    { row: number; col: number; layer: number } | null
  >(null);
  const selectedKeymap = selectedKey
    ? props.keymaps.find((v) =>
      v.col == selectedKey.col && v.row == selectedKey.row &&
      v.layer == selectedKey.layer
    )
    : null;

  return (
    <div className="flex flex-col">
      <div className="relative h-72">
        {kb.keys.map((key) => (
          <div
            className="absolute border-2 border-black"
            style={{
              width: key.width * 50 + "px",
              height: key.height * 50 + "px",
              top: key.y * 50 + "px",
              left: key.x * 50 + "px",
              transform: `rotate(${key.rotation_angle}deg)`,
            }}
            onClick={() => {
              const loc = key.labels[0];
              const [row, col] = loc.split(",").map((x) => parseInt(x));
              setSelectedKey({ row, col, layer: 0 });
            }}
          >
            {displayKey(
              props.keymaps.find((v) =>
                v.col == parseInt(key.labels[0].split(",")[1]) &&
                v.row == parseInt(key.labels[0].split(",")[0]) &&
                v.layer == 0
              )?.key,
            )}
          </div>
        ))}
      </div>
      <div>
        {selectedKey
          ? (
            <div>
              <h2>Selected key</h2>
              <p>Row: {selectedKey.row}</p>
              <p>Col: {selectedKey.col}</p>
              <p>Layer: {selectedKey.layer}</p>
              <p>
                Keymap:{" "}
                {selectedKeymap ? JSON.stringify(selectedKeymap.key) : "None"}
              </p>
            </div>
          )
          : <div>No key selected</div>}
      </div>
    </div>
  );
}

function displayKey(key?: KeyDef): string {
  if (!key) {
    return "";
  }
  if (typeof key !== "string" && "Key" in key) {
    if ("Normal" in key.Key) {
      if ("Key" in key.Key.Normal) {
        return key.Key.Normal.Key;
      }
    }
  }

  return "";
}

function KeyChanger(props: { keydef: KeyDef }) {
  const [keydef, setKeydef] = useState<KeyDef>(props.keydef);

  let childMenu = null;
  let active = null;

  switch (keydef) {
    case "None":
      active = "None";
      break;
    case "Inherit":
      active = "Inherit";
      break;
    default:
      active = "Custom";
      childMenu = (
        <KeyActionMenu
          keyAction={keydef.Key}
          setKeyAction={(ka) => setKeydef({ Key: ka })}
        />
      );
  }

  return (
    <div>
      <NavLink>
        <NavLink active={active == "None"}>None</NavLink>
        <NavLink active={active == "Inherit"}>None</NavLink>
        <NavLink active={active == "Custom"}></NavLink>
      </NavLink>
    </div>
  );
}

function KeyActionMenu(
  props: { keyAction: KeyAction; setKeyAction: (keyAction: KeyAction) => void },
) {
  return (
    <div>
    </div>
  );
}
