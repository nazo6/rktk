import { Button, Title3 } from "@fluentui/react-components";
import { keyStr } from "../../lib/keyStr";
import { KeyUpdate } from "./types";

export function KeyUpdater(props: {
  toUpdateKeyActions: { current: KeyUpdate; new: KeyUpdate }[];
  updateKeymap: (keymap: KeyUpdate[]) => void;
  clearKeymapModifications: () => void;
}) {
  return (
    <div className="w-full">
      <div className="p-2 bg-gray-300/50 rounded-md flex flex-col">
        <Title3>Write Queue</Title3>
        <div className="flex flex-col">
          {props.toUpdateKeyActions.map((action, i) => (
            <div key={i}>
              <p>
                {keyStr(action.current.action)} {" -> "}{" "}
                {keyStr(action.new.action)}
              </p>
            </div>
          ))}
        </div>
        <div className="w-full flex gap-2">
          <Button
            className="flex-grow"
            onClick={() => {
              props.updateKeymap(
                props.toUpdateKeyActions.map((a) => a.new),
              );
            }}
          >
            Write
          </Button>
          <Button
            className="flex-grow"
            color="red"
            onClick={props.clearKeymapModifications}
          >
            Clear
          </Button>
        </div>
      </div>
    </div>
  );
}
