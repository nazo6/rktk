import { Divider, Radio, RadioGroup, Select } from "@fluentui/react-components";
import { KeyAction, KeyCode } from "../../../bindings";
import {
  MEDIA_KEYS,
  MODIFIER_KEYS,
  MOUSE_KEYS,
  NORMAL_KEYS,
  SPECIAL_KEYS,
} from "../keys";

export function KeyActionSelector(props: {
  keyAction: KeyAction;
  setKeyAction: (keyAction: KeyAction) => void;
}) {
  let kcSelector;
  if (props.keyAction == "Inherit") {
    kcSelector = <div>Inherit</div>;
  } else if ("Normal" in props.keyAction) {
    kcSelector = (
      <KeyCodeSelector
        keycode={props.keyAction.Normal}
        setKeycode={(keycode) => props.setKeyAction({ Normal: keycode })}
      />
    );
  } else if ("Normal2" in props.keyAction) {
    kcSelector = (
      <div className="flex flex-wrap gap-2">
        <KeyCodeSelector
          keycode={props.keyAction.Normal2[0]}
          setKeycode={(keycode) => {
            if (
              typeof props.keyAction != "string" && "Normal2" in props.keyAction
            ) {
              props.setKeyAction({
                Normal2: [keycode, props.keyAction.Normal2[1]],
              });
            }
          }}
        />
        <KeyCodeSelector
          keycode={props.keyAction.Normal2[1]}
          setKeycode={(keycode) => {
            if (
              typeof props.keyAction != "string" && "Normal2" in props.keyAction
            ) {
              props.setKeyAction({
                Normal2: [props.keyAction.Normal2[0], keycode],
              });
            }
          }}
        />
      </div>
    );
  } else {
    kcSelector = <div>Not implemented</div>;
  }

  return (
    <div>
      <div className="flex flex-col">
        <div className="flex items-center">
          KeyAction:
          <RadioGroup layout="horizontal">
            <Radio checked={props.keyAction == "Inherit"} label="Inherit" />
            <Radio
              checked={typeof props.keyAction != "string" &&
                "Normal" in props.keyAction}
              label="Normal"
            />
            <Radio
              checked={typeof props.keyAction != "string" &&
                "Normal2" in props.keyAction}
              label="Normal2"
            />
            <Radio
              checked={typeof props.keyAction != "string" &&
                "TapHold" in props.keyAction}
              label="TapHold"
            />
            <Radio
              checked={typeof props.keyAction != "string" &&
                "OneShot" in props.keyAction}
              label="OneShot"
            />
            <Radio
              checked={typeof props.keyAction != "string" &&
                "TapDance" in props.keyAction}
              label="TapDance"
            />
          </RadioGroup>
        </div>
        <Divider />
        {kcSelector}
      </div>
    </div>
  );
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
      <div className="flex items-center">
        KeyCode:
        <RadioGroup layout="horizontal">
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
            checked={typeof props.keycode != "string" &&
              "Mouse" in props.keycode}
            label="Mouse"
          />
          <Radio
            checked={typeof props.keycode != "string" &&
              "Media" in props.keycode}
            label="Media"
          />
          <Radio
            checked={typeof props.keycode != "string" &&
              "Layer" in props.keycode}
            label="Layer"
          />
          <Radio
            checked={typeof props.keycode != "string" &&
              "Special" in props.keycode}
            label="Special"
          />
        </RadioGroup>
      </div>
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
      <Select value={props.keys.get(props.selected)}>
        {Array.from(props.keys.entries()).map(([key, label]) => (
          <option key={String(key)}>
            {label}
          </option>
        ))}
      </Select>
    </div>
  );
}
