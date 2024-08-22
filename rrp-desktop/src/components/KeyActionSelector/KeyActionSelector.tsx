import { Input, Radio, RadioGroup } from "@fluentui/react-components";
import { KeyAction, KeyCode } from "../../bindings";
import { KeyCodeSelector } from "./KeyCodeSelector";

export function KeyActionSelector(props: {
  keyAction: KeyAction;
  layerCount: number;
  setKeyAction: (keyAction: KeyAction) => void;
}) {
  let kcSelector;
  if (props.keyAction == "Inherit") {
    kcSelector = <div></div>;
  } else if ("Normal" in props.keyAction) {
    kcSelector = (
      <KeyCodeSelector
        layerCount={props.layerCount}
        keycode={props.keyAction.Normal}
        setKeycode={(keycode) => props.setKeyAction({ Normal: keycode })}
      />
    );
  } else if ("Normal2" in props.keyAction) {
    kcSelector = (
      <Normal2Selector
        layerCount={props.layerCount}
        keyCode1={props.keyAction.Normal2[0]}
        keyCode2={props.keyAction.Normal2[1]}
        setKeyAction={props.setKeyAction}
      />
    );
  } else if ("TapHold" in props.keyAction) {
    kcSelector = (
      <TapHoldSelector
        layerCount={props.layerCount}
        keyCodeTap={props.keyAction.TapHold[0]}
        keyCodeHold={props.keyAction.TapHold[1]}
        setKeyAction={props.setKeyAction}
      />
    );
  } else if ("OneShot" in props.keyAction) {
    kcSelector = (
      <KeyCodeSelector
        layerCount={props.layerCount}
        keycode={props.keyAction.OneShot}
        setKeycode={(keycode) => props.setKeyAction({ OneShot: keycode })}
      />
    );
  } else if ("TapDance" in props.keyAction) {
    kcSelector = (
      <NumberSelector
        number={props.keyAction.TapDance}
        setNumber={(number) => props.setKeyAction({ TapDance: number })}
      />
    );
  } else {
    kcSelector = <div>Not implemented</div>;
  }

  return (
    <div className="flex flex-col">
      <div className="flex items-center">
        KeyAction:
        <RadioGroup
          layout="horizontal"
          className="flex-wrap"
          value={typeof props.keyAction == "string"
            ? props.keyAction
            : Object.keys(props.keyAction)[0]}
          onChange={(_, data) => {
            switch (data.value) {
              case "Inherit":
                props.setKeyAction("Inherit");
                break;
              case "Normal":
                props.setKeyAction({ Normal: "None" });
                break;
              case "Normal2":
                props.setKeyAction({ Normal2: ["None", "None"] });
                break;
              case "TapHold":
                props.setKeyAction({ TapHold: ["None", "None"] });
                break;
              case "OneShot":
                props.setKeyAction({ OneShot: "None" });
                break;
              case "TapDance":
                props.setKeyAction({ TapDance: 0 });
                break;
            }
          }}
        >
          <Radio value="Inherit" label="Inherit" />
          <Radio value="Normal" label="Normal" />
          <Radio value="Normal2" label="Normal2" />
          <Radio value="TapHold" label="TapHold" />
          <Radio value="OneShot" label="OneShot" />
          <Radio value="TapDance" label="TapDance" />
        </RadioGroup>
      </div>
      {kcSelector}
    </div>
  );
}

function NumberSelector(
  props: {
    number: number;
    setNumber: (number: number) => void;
  },
) {
  return (
    <Input
      type="number"
      value={props.number as any}
      onChange={(e) => {
        props.setNumber(e.target.value as any);
      }}
    />
  );
}

function Normal2Selector(
  props: {
    keyCode1: KeyCode;
    keyCode2: KeyCode;
    layerCount: number;
    setKeyAction: (keyAction: KeyAction) => void;
  },
) {
  return (
    <div className="flex flex-col flex-wrap gap-2">
      <KeyCodeSelector
        title="First keycode"
        layerCount={props.layerCount}
        keycode={props.keyCode1}
        setKeycode={(keycode) => {
          props.setKeyAction({ Normal2: [keycode, props.keyCode2] });
        }}
      />
      <KeyCodeSelector
        title="Second keycode"
        layerCount={props.layerCount}
        keycode={props.keyCode2}
        setKeycode={(keycode) => {
          props.setKeyAction({ Normal2: [props.keyCode1, keycode] });
        }}
      />
    </div>
  );
}

function TapHoldSelector(
  props: {
    keyCodeTap: KeyCode;
    keyCodeHold: KeyCode;
    layerCount: number;
    setKeyAction: (keyAction: KeyAction) => void;
  },
) {
  return (
    <div className="flex flex-col flex-wrap gap-2">
      <KeyCodeSelector
        title="Tap keycode"
        layerCount={props.layerCount}
        keycode={props.keyCodeTap}
        setKeycode={(keycode) => {
          props.setKeyAction({ TapHold: [keycode, props.keyCodeHold] });
        }}
      />
      <KeyCodeSelector
        title="Hold keycode"
        layerCount={props.layerCount}
        keycode={props.keyCodeHold}
        setKeycode={(keycode) => {
          props.setKeyAction({ TapHold: [props.keyCodeTap, keycode] });
        }}
      />
    </div>
  );
}
