import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  Toast,
  ToastBody,
  ToastTitle,
  useToastController,
} from "@fluentui/react-components";
import { atom, useAtom, useSetAtom } from "jotai";
import { Client, KeyboardInfo } from "rrp-client-web";

export type Connection = {
  keyboard: KeyboardInfo;
  port: SerialPort;
  client: Client;
};

export const connectionAtom = atom<Connection | null>(null);

export function useDisconnect() {
  const { dispatchToast } = useToastController();
  const queryClient = useQueryClient();

  const [connection, setConnection] = useAtom(connectionAtom);

  return useMutation({
    mutationFn: async (_notify: boolean = true) => {
      if (connection) {
        connection.client.free();
        await connection.port.close();
        setConnection(null);
      } else {
        throw new Error("No connection to disconnect");
      }
    },
    onSuccess: (_, notify) => {
      queryClient.invalidateQueries({ queryKey: ["getKeymaps"] });
      queryClient.invalidateQueries({ queryKey: ["getLayoutJson"] });
      queryClient.invalidateQueries({ queryKey: ["getKeymapConfig"] });
      if (!notify) return;
      dispatchToast(
        <Toast>
          <ToastTitle>
            Disconnected serial
          </ToastTitle>
        </Toast>,
        { intent: "success" },
      );
    },
    onError: (e, notify) => {
      if (!notify) return;
      dispatchToast(
        <Toast>
          <ToastTitle>
            Error disconnecting serial
          </ToastTitle>
          <ToastBody>
            {e instanceof Error ? e.message : "Unknown error"}
          </ToastBody>
        </Toast>,
        { intent: "error" },
      );
    },
  });
}

export function useConnect() {
  const { dispatchToast } = useToastController();

  const setConnection = useSetAtom(connectionAtom);
  const disconnect = useDisconnect();

  return useMutation({
    mutationFn: async () => {
      const port = await navigator.serial.requestPort();
      await port.open({ baudRate: 115200 });
      const client = new Client(port);
      try {
        const kb = await client.get_keyboard_info();
        setConnection({
          client,
          port,
          keyboard: kb,
        });
        return kb;
      } catch (e) {
        try {
          await port.close();
        } catch (e2) {
          throw e + "\n" + e2;
        }
        throw e;
      }
    },
    onSuccess: (kb) => {
      dispatchToast(
        <Toast>
          <ToastTitle>
            Connected to {kb.name}
          </ToastTitle>
        </Toast>,
        { intent: "success" },
      );
    },
    onError: (e) => {
      disconnect.mutate(false);
      dispatchToast(
        <Toast>
          <ToastTitle>
            Error connecting to keyboard. Disconnecting.
          </ToastTitle>
          <ToastBody>
            {typeof e === "string"
              ? e
              : e instanceof Error
              ? e.message
              : "Unknown error"}
          </ToastBody>
        </Toast>,
        { intent: "error" },
      );
    },
  });
}
