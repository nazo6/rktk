import { useMutation } from "@tanstack/react-query";
import { unwrapped } from "../utils";
import { commands } from "../bindings";
import {
  Toast,
  ToastBody,
  ToastTitle,
  useToastController,
} from "@fluentui/react-components";

export function useDisconnect() {
  const { dispatchToast } = useToastController();

  return useMutation({
    mutationFn: async (_notify: boolean = true) =>
      unwrapped(commands.disconnect)(),
    onSuccess: (_, notify) => {
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

  const disconnect = useDisconnect();

  return useMutation({
    mutationFn: async (
      port: string,
    ) => {
      await unwrapped(commands.connect)(port);
      return await unwrapped(commands.getKeyboardInfo)();
    },
    onSuccess: (info, opts) => {
      dispatchToast(
        <Toast>
          <ToastTitle>
            Connected to {info.name}
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
            {e instanceof Error ? e.message : "Unknown error"}
          </ToastBody>
        </Toast>,
        { intent: "error" },
      );
    },
  });
}
