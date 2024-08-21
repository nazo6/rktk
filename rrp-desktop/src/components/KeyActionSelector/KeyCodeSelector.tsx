import { Radio, RadioGroup } from "@fluentui/react-components";
import { KeyCode } from "../../bindings";
import {
  MEDIA_KEYS,
  MODIFIER_KEYS,
  MOUSE_KEYS,
  NORMAL_KEYS,
  SPECIAL_KEYS,
} from "../../lib/keys";
import { KeySelector, LayerKeySelector } from "./KeySelector";

export function KeyCodeSelector(props: {
  title?: string;
  keycode: KeyCode;
  layerCount: number;
  setKeycode: (keycode: KeyCode) => void;
}) {
  let keySelector;
  if (props.keycode == "None") {
    keySelector = <div></div>;
  } else {
    if ("Key" in props.keycode) {
      keySelector = (
        <KeySelector
          keys={NORMAL_KEYS}
          selected={props.keycode.Key}
          setSelected={(key) => props.setKeycode({ Key: key })}
        />
      );
    } else if ("Mouse" in props.keycode) {
      keySelector = (
        <KeySelector
          keys={MOUSE_KEYS}
          selected={props.keycode.Mouse}
          setSelected={(key) => props.setKeycode({ Mouse: key })}
        />
      );
    } else if ("Modifier" in props.keycode) {
      keySelector = (
        <KeySelector
          keys={MODIFIER_KEYS}
          selected={props.keycode.Modifier}
          setSelected={(key) => props.setKeycode({ Modifier: key })}
        />
      );
    } else if ("Media" in props.keycode) {
      keySelector = (
        <KeySelector
          keys={MEDIA_KEYS}
          selected={props.keycode.Media}
          setSelected={(key) => props.setKeycode({ Media: key })}
        />
      );
    } else if ("Layer" in props.keycode) {
      keySelector = (
        <LayerKeySelector
          layerCount={props.layerCount}
          selected={props.keycode.Layer}
          setSelected={(key) => props.setKeycode({ Layer: key })}
        />
      );
    } else if ("Special" in props.keycode) {
      keySelector = (
        <KeySelector
          keys={SPECIAL_KEYS}
          selected={props.keycode.Special}
          setSelected={(key) => props.setKeycode({ Special: key })}
        />
      );
    }
  }

  return (
    <div className="flex flex-col bg-gray-100/30 p-2 rounded-md">
      <div className="flex items-center">
        {props.title ?? "Keycode"}:
        <RadioGroup
          layout="horizontal"
          className="flex-wrap"
          value={typeof props.keycode == "string"
            ? props.keycode
            : Object.keys(props.keycode)[0]}
          onChange={(_, data) => {
            switch (data.value) {
              case "None":
                props.setKeycode("None");
                break;
              case "Key":
                props.setKeycode({ Key: "A" });
                break;
              case "Mouse":
                props.setKeycode({ Mouse: 0x01 });
                break;
              case "Modifier":
                props.setKeycode({ Modifier: 0x01 });
                break;
              case "Layer":
                props.setKeycode({ Layer: { Toggle: 1 } });
                break;
              case "Special":
                props.setKeycode({ Special: "MoScrl" });
                break;
              case "Media":
                props.setKeycode({ Media: "Play" });
                break;
            }
          }}
        >
          <Radio value="None" label="None" />
          <Radio value="Key" label="Key" />
          <Radio value="Mouse" label="Mouse" />
          <Radio value="Modifier" label="Modifier" />
          <Radio value="Layer" label="Layer" />
          <Radio value="Special" label="Special" />
          <Radio value="Media" label="Media" />
        </RadioGroup>
      </div>
      {keySelector}
    </div>
  );
}
