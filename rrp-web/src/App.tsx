import { Home } from "./page/Home";
import { Connect } from "./page/Connect";
import { Button, Toaster } from "@fluentui/react-components";
import { TitleBar } from "./components/TitleBar";
import { useAtomValue } from "jotai";
import { connectionAtom, useDisconnect } from "./lib/connection";

export const TOASTER_ID = "toaster";

export default function App() {
  const connection = useAtomValue(connectionAtom);

  const serialSupported = !!navigator.serial;

  const disconnect = useDisconnect();

  return (
    <div className="flex flex-col h-full">
      <TitleBar>
        {connection && (
          <div className="flex ml-auto items-center gap-2">
            <div>
              Connected to{" "}
              <span className="font-bold text-[1rem]">
                {connection.keyboard.name}
              </span>
            </div>
            <Button
              appearance="secondary"
              onClick={() => disconnect.mutate(true)}
            >
              Disconnect
            </Button>
          </div>
        )}
        <Toaster position="bottom-end" />
      </TitleBar>
      <div className="min-h-0 flex-grow">
        {serialSupported
          ? connection ? <Home connection={connection} /> : <Connect />
          : (
            <div className="flex justify-center items-center h-full text-lg p-2">
              Sorry, this browser does not support web serial, so rrp-client
              cannot be used. Please use browser that supports web serial, such
              as Chrome or Edge browser.
            </div>
          )}
      </div>
    </div>
  );
}
