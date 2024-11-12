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
  device: HIDDevice;
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
        await connection.device.close();
        setConnection(null);
      } else {
        throw new Error("No connection to disconnect");
      }
    },
    onSuccess: (_, notify) => {
      queryClient.resetQueries({ queryKey: ["getKeymaps"] });
      queryClient.resetQueries({ queryKey: ["getLayoutJson"] });
      queryClient.resetQueries({ queryKey: ["getKeymapConfig"] });
      queryClient.resetQueries({ queryKey: ["getLog"] });
      queryClient.resetQueries({ queryKey: ["getNow"] });
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
      const devices = await navigator.hid.requestDevice({
        filters: [{
          usagePage: 0xFF70,
          usage: 0x71,
        }],
      });
      console.log(devices);
      if (devices.length === 0) {
        throw new Error("No devices found");
      }
      const device = devices[0];
      const client = new Client(device);
      await device.open();
      try {
        const kb = await client.get_keyboard_info();
        console.log(kb);
        const layout = await client.get_layout_json();
        console.log(layout);
        // setConnection({
        //   client,
        //   device,
        //   keyboard: kb,
        // });
        return kb;
      } catch (e) {
        try {
          await device.close();
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
      console.error(e);
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
