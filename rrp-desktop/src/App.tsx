import { useState } from "react";
import { KeyboardInfo } from "./bindings";
import { Home } from "./page/Home";
import { Connect } from "./page/Connect";
import { Button, Toaster } from "@fluentui/react-components";
import { TitleBar } from "./components/TitleBar";
import { useDisconnect } from "./lib/connectHook";

export const TOASTER_ID = "toaster";

export default function App() {
  const [keyboardInfo, setKeyboardInfo] = useState<KeyboardInfo | null>(null);

  const disconnect = useDisconnect();

  return (
    <div className="flex flex-col h-full">
      <TitleBar>
        {keyboardInfo && (
          <div className="flex ml-auto items-center gap-2">
            <div>
              Connected to:{" "}
              <span className="font-bold text-[1rem]">{keyboardInfo.name}</span>
            </div>
            <Button
              appearance="secondary"
              onClick={async () => {
                await disconnect.mutateAsync(true);
                setKeyboardInfo(null);
              }}
            >
              Disconnect
            </Button>
          </div>
        )}
      </TitleBar>
      <Toaster position="bottom-end" />
      {keyboardInfo
        ? (
          <Home
            keyboardInfo={keyboardInfo}
          />
        )
        : (
          <Connect
            setKeyboardInfo={setKeyboardInfo}
          />
        )}
    </div>
  );
}
