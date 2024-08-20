import * as kle from "@ijprest/kle-serial";
import { KeyAction, KeyActionLoc } from "../../../bindings";
import { KeyData } from "..";
import {
  Button,
  Popover,
  PopoverSurface,
  PopoverTrigger,
  ToggleButton,
  Toolbar,
  ToolbarRadioButton,
  ToolbarRadioGroup,
} from "@fluentui/react-components";
import { KeyActionSelector } from "./KeySelector";
import { useState } from "react";

export function Keyboard(
  props: {
    keys: KeyData[];
    layers: number;
    updateKeymap: (changes: KeyActionLoc[]) => Promise<void>;
  },
) {
  const [layer, setLayer] = useState(0);
  const [selectedKey, setSelectedKey] = useState<KeyActionLoc | null>(null);

  return (
    <div>
      <div className="flex gap-2">
        <div className="flex flex-col">
          {Array.from(Array(props.layers)).map((_, i) => (
            <ToggleButton
              appearance="subtle"
              key={i}
              checked={i == layer}
              onClick={() => setLayer(i)}
              icon={<span>{i}</span>}
            />
          ))}
        </div>
        <div className="relative w-auto h-auto">
          {props.keys.map((keydata, i) => (
            <Key
              kleKey={keydata.kleKey}
              keyAction={keydata.actions?.[layer]}
              key={i}
              onClick={(ka) => {
                setSelectedKey(ka);
              }}
            />
          ))}
        </div>
      </div>
      <div className="fixed bottom-0 w-full">
        <div className=" mb-9 mx-3 p-2 bg-gray-300/50 rounded-md">
          {selectedKey && (
            <div className="flex flex-col">
              <div className="flex gap-3">
                <p>Row: {selectedKey.row}</p>
                <p>Col: {selectedKey.col}</p>
                <p>Layer: {selectedKey.layer}</p>
              </div>
              <p>{JSON.stringify(selectedKey.key)}</p>
              <KeyActionSelector
                keyAction={selectedKey.key}
                setKeyAction={(ka) => {}}
              />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function Key(
  { kleKey, keyAction, onClick }: {
    kleKey: kle.Key;
    keyAction: KeyActionLoc;
    onClick?: (action: KeyActionLoc) => void;
  },
) {
  return (
    <div
      className="absolute border-2 border-black"
      style={{
        width: kleKey.width * 50 + "px",
        height: kleKey.height * 50 + "px",
        top: kleKey.y * 50 + "px",
        left: kleKey.x * 50 + "px",
        transform: `rotate(${kleKey.rotation_angle}deg)`,
      }}
      onClick={() => onClick?.(keyAction)}
    >
      {keyStr(keyAction?.key)}
    </div>
  );
}

function keyStr(key?: KeyAction): string {
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
