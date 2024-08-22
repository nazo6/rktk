import { keyStr } from "@/lib/keyStr";
import clsx from "clsx";
import { deepEqual } from "fast-equals";
import { KeyData, KeyLoc } from "../types";
import { SIZE_MULTIPLIER } from ".";

export function KeyboardView(
  props: {
    keys: KeyData[];
    layer: number;
    selectKeyLoc: (key: KeyLoc) => void;
    selectedKeyLoc: KeyLoc | null;
    style?: React.CSSProperties;
  },
) {
  return (
    <div
      className="px-2"
      style={props.style}
    >
      <div className="relative w-auto h-auto">
        {props.keys
          .filter((keyData) => keyData.loc.layer == props.layer)
          .map((keyData, i) => {
            return (
              <Key
                key={i}
                keyData={keyData}
                onClick={() => props.selectKeyLoc(keyData.loc)}
                selected={deepEqual(keyData.loc, props.selectedKeyLoc)}
              />
            );
          })}
      </div>
    </div>
  );
}

function Key(
  { keyData, onClick, selected }: {
    keyData: KeyData;
    onClick?: (action: KeyData) => void;
    selected: boolean;
  },
) {
  return (
    <div
      className={clsx(
        "absolute border-2 p-1 font-bold cursor-pointer hover:bg-gray-500/20",
        keyData.changed && "text-red-500",
        selected && "border-blue-500 dark:border-blue-400",
        !selected && "border-black dark:border-gray-500",
      )}
      style={{
        width: keyData.kleKey.width * SIZE_MULTIPLIER - 3 + "px",
        height: keyData.kleKey.height * SIZE_MULTIPLIER - 3 + "px",
        top: keyData.kleKey.y * SIZE_MULTIPLIER + "px",
        left: keyData.kleKey.x * SIZE_MULTIPLIER + "px",
        transform: `rotate(${keyData.kleKey.rotation_angle}deg)`,
      }}
      onClick={() => onClick?.(keyData)}
    >
      {keyStr(keyData.action)}
    </div>
  );
}
