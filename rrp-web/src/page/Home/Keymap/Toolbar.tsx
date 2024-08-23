import {
  Button,
  Popover,
  PopoverSurface,
  PopoverTrigger,
} from "@fluentui/react-components";
import { keyStr } from "@/lib/keyStr";
import { KeyUpdate } from "./types";

export function Toolbar(props: {
  toUpdateKeyActions: { crr: KeyUpdate; new: KeyUpdate }[];
  updateKeymap: (keymap: KeyUpdate[]) => void;
  clearKeymapModifications: () => void;
}) {
  return (
    <div className="w-full flex px-2 py-1 h-8 bg-blue-500/10">
      <div>
      </div>
      <div className="flex ml-auto gap-2">
        {props.toUpdateKeyActions.length > 0 && (
          <>
            <Popover>
              <PopoverTrigger>
                <Button className="flex-grow" appearance="outline">
                  {props.toUpdateKeyActions.length}
                  {" pending write(s)"}
                </Button>
              </PopoverTrigger>
              <PopoverSurface>
                <div className="flex flex-col">
                  {props.toUpdateKeyActions.map((action, i) => (
                    <div key={i}>
                      <p>
                        {keyStr(action.crr.action)} {" -> "}{" "}
                        {keyStr(action.new.action)}
                      </p>
                    </div>
                  ))}
                </div>
              </PopoverSurface>
            </Popover>
            <Button
              className="flex-grow"
              onClick={props.clearKeymapModifications}
            >
              Clear
            </Button>
            <Button
              appearance="primary"
              className="flex-grow"
              onClick={() => {
                props.updateKeymap(
                  props.toUpdateKeyActions.map((a) => a.new),
                );
              }}
            >
              Write
            </Button>
          </>
        )}
      </div>
    </div>
  );
}
