import { KeyAction } from "../../../bindings";
import { KeyData } from "./types";
import { KeyActionSelector } from "../../../components/KeyActionSelector/KeyActionSelector";
import { Button } from "@fluentui/react-components";

export function KeySelector(
  props: {
    selectedKey?: KeyData | null;
    onChange: (action: KeyAction) => void;
    restoreSelectedKey: () => void;
    layerCount: number;
  },
) {
  return (
    <div className="bg-gray-300/50 rounded-md p-2 max-w-3xl w-full">
      {props.selectedKey && (
        <div className="flex flex-col gap-1">
          <p className="text-lg font-bold">Selected key</p>
          <div className="flex gap-3 items-center h-8">
            <p>Row: {props.selectedKey.loc.row}</p>
            <p>Col: {props.selectedKey.loc.col}</p>
            <p>Layer: {props.selectedKey.loc.layer}</p>
            {props.selectedKey.changed && (
              <Button onClick={props.restoreSelectedKey}>
                Restore this key
              </Button>
            )}
          </div>
          <KeyActionSelector
            layerCount={props.layerCount}
            keyAction={props.selectedKey.action}
            setKeyAction={props.onChange}
          />
        </div>
      )}
    </div>
  );
}
