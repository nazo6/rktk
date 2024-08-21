import { useState } from "react";
import { KeyboardInfo, SerialPortInfo } from "./bindings";
import { Home } from "./page/Home";
import { Connect } from "./page/Connect";
import { Button, Toaster } from "@fluentui/react-components";
import { TitleBar } from "./components/TitleBar";
import { useDisconnect } from "./lib/connectHook";

export const TOASTER_ID = "toaster";

export type ConnectionInfo = {
  keyboard: KeyboardInfo;
  port: SerialPortInfo;
};

export default function App() {
  const [connectionInfo, setKeyboardInfo] = useState<ConnectionInfo | null>(
    null,
  );

  const disconnect = useDisconnect();

  return (
    <div className="flex flex-col h-full">
      <TitleBar>
        {connectionInfo && (
          <div className="flex ml-auto items-center gap-2">
            <div>
              Connected to{" "}
              <span className="font-bold text-[1rem]">
                {connectionInfo.keyboard.name}
              </span>
              {" at "}
              <span className="font-bold">
                {connectionInfo.port.port_name}
              </span>
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
        <Toaster position="bottom-end" />
      </TitleBar>
      <div className="min-h-0 flex-grow">
        {connectionInfo
          ? (
            <Home
              keyboardInfo={connectionInfo.keyboard}
            />
          )
          : (
            <Connect
              setConnectionInfo={setKeyboardInfo}
            />
          )}
      </div>
    </div>
  );
}
