import { Divider, Radio, RadioGroup } from "@fluentui/react-components";
import { KeyAction } from "../../bindings";
import { KeyCodeSelector } from "./KeyCodeSelector";

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
          <RadioGroup layout="horizontal" className="flex-wrap">
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
