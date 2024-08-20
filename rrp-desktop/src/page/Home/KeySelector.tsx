import { KeyAction } from "../../bindings";
import { KeyData } from "./types";
import { KeyActionSelector } from "../../components/KeyActionSelector/KeyActionSelector";

export function KeySelector(
  { selectedKey, onChange }: {
    selectedKey?: KeyData;
    onChange: (action: KeyAction) => void;
  },
) {
  return (
    <div className="bg-gray-300/50 rounded-md p-2">
      {selectedKey && (
        <div className="flex flex-col">
          <div className="flex gap-3">
            <p>Row: {selectedKey.loc.row}</p>
            <p>Col: {selectedKey.loc.col}</p>
            <p>Layer: {selectedKey.loc.layer}</p>
          </div>
          <p>{JSON.stringify(selectedKey.action)}</p>
          <KeyActionSelector
            keyAction={selectedKey.action}
            setKeyAction={onChange}
          />
        </div>
      )}
    </div>
  );
}
