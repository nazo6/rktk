import { Home } from "./page/Home";
import { Connect } from "./page/Connect";
import { Button, Toaster } from "@fluentui/react-components";
import { TitleBar } from "./components/TitleBar";
import { atom, useAtomValue } from "jotai";
import { connectionAtom, useDisconnect } from "./lib/connection";
import { QueryClientProvider } from "@tanstack/react-query";
import {
  FluentProvider,
  webDarkTheme,
  webLightTheme,
} from "@fluentui/react-components";
import { queryClient } from "./queryClient";
import { useEffect } from "react";

export const TOASTER_ID = "toaster";

export const themeAtom = atom<"light" | "dark" | "system">(
  localStorage.getItem("theme") as "light" | "dark" | "system" ?? "system",
);

export function Providers() {
  const theme = useAtomValue(themeAtom);
  const themeResolved = theme === "system"
    ? window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light"
    : theme;

  useEffect(() => {
    localStorage.setItem("theme", theme);
  }, [theme]);

  useEffect(() => {
    if (themeResolved === "dark") {
      document.body.classList.add("dark");
    } else {
      document.body.classList.remove("dark");
    }
  }, [themeResolved]);

  return (
    <QueryClientProvider client={queryClient}>
      <FluentProvider
        theme={themeResolved == "dark" ? webDarkTheme : webLightTheme}
      >
        <div className="h-[100vh]">
          <App />
        </div>
      </FluentProvider>
    </QueryClientProvider>
  );
}

function App() {
  const connection = useAtomValue(connectionAtom);

  const serialSupported = !!navigator.serial;

  const disconnect = useDisconnect();

  return (
    <div className="flex flex-col h-full">
      <TitleBar>
        {connection
          ? (
            <div className="flex items-center gap-2">
              <div>
                Connected to:
                <span className="font-bold text-lg pl-1">
                  {connection.keyboard.name}
                </span>
              </div>
              <Button
                appearance="secondary"
                size="small"
                onClick={() => disconnect.mutate(true)}
              >
                Disconnect
              </Button>
            </div>
          )
          : "Disconnected"}
      </TitleBar>
      <div className="min-h-0 flex-grow">
        <Toaster position="bottom-end" />
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
