import { Group, Radio, Select } from "@mantine/core";
import { KeyAction, KeyCode } from "../../bindings";
import {
  MEDIA_KEYS,
  MODIFIER_KEYS,
  MOUSE_KEYS,
  NORMAL_KEYS,
  SPECIAL_KEYS,
} from "./keys";

export function KeyActionSelector(props: {
  keyAction: KeyAction;
  setKeyAction: (keyAction: KeyAction) => void;
}) {
  return (typeof props.keyAction != "string" &&
      "Normal" in props.keyAction)
    ? (
      <KeyCodeSelector
        keycode={props.keyAction.Normal}
        setKeycode={(keycode) => props.setKeyAction({ Normal: keycode })}
      />
    )
    : <div>Not implemented</div>;
}

export function KeyCodeSelector(props: {
  keycode: KeyCode;
  setKeycode: (keycode: KeyCode) => void;
}) {
  let keySelector;
  if (props.keycode == "None") {
    keySelector = <div>None</div>;
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
        <div>
          Not implemented
        </div>
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
    <div className="flex flex-col">
      <Group>
        <Radio checked={props.keycode == "None"} label="None" />
        <Radio
          checked={typeof props.keycode != "string" && "Key" in props.keycode}
          label="Key"
        />
        <Radio
          checked={typeof props.keycode != "string" &&
            "Modifier" in props.keycode}
          label="Modifier"
        />
        <Radio
          checked={typeof props.keycode != "string" && "Mouse" in props.keycode}
          label="Mouse"
        />
        <Radio
          checked={typeof props.keycode != "string" && "Media" in props.keycode}
          label="Media"
        />
        <Radio
          checked={typeof props.keycode != "string" &&
            "Special" in props.keycode}
          label="Special"
        />
      </Group>
      {keySelector}
    </div>
  );
}

function KeySelector<T>(
  props: {
    keys: Map<T, string>;
    selected: T;
    setSelected: (key: T) => void;
  },
) {
  return (
    <div className="flex flex-col">
      <Select
        placeholder="Pick value"
        value={props.keys.get(props.selected)}
        data={[...props.keys.entries()]
          .map(([key, value]) => value)}
      />
    </div>
  );
}
