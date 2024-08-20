import { useRef } from "react";
import { KeyData, KeyLoc } from "../types";
import { KeyboardView } from "./KeyboardView";
import { LayerSelector } from "./LayerSelector";
import useSize from "@react-hook/size";

export const SIZE_MULTIPLIER = 60;

export function Keyboard(
  props: {
    keys: KeyData[];
    layer: number;
    layerCount: number;
    setLayer: (layer: number) => void;
    selectKeyLoc: (key: KeyLoc) => void;
    selectedKeyLoc: KeyLoc | null;
  },
) {
  const keyboardWidth = props.keys.reduce(
        (acc, key) => Math.max(acc, key.kleKey.x + key.kleKey.width),
        0,
      ) * SIZE_MULTIPLIER + 30;
  const keyboardHeight = props.keys.reduce(
        (acc, key) => Math.max(acc, key.kleKey.y + key.kleKey.height),
        0,
      ) * SIZE_MULTIPLIER + 30;

  let containerRef = useRef<HTMLDivElement>(null);
  const [parentWidth] = useSize(containerRef);
  const scale = Math.min((parentWidth - 100) / keyboardWidth, 1);

  return (
    <div
      className="flex items-center justify-center w-full p-2"
      ref={containerRef}
    >
      <LayerSelector
        layer={props.layer}
        setLayer={props.setLayer}
        layerCount={props.layerCount}
      />
      <KeyboardView
        keys={props.keys}
        layer={props.layer}
        selectKeyLoc={(key) => props.selectKeyLoc(key)}
        selectedKeyLoc={props.selectedKeyLoc}
        style={{
          transform: `scale(${scale})`,
          transformOrigin: "top left",
          width: keyboardWidth * scale + "px",
          height: keyboardHeight * scale + "px",
        }}
      />
    </div>
  );
}
