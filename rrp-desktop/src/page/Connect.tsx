import { Button } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { useEffect, useState } from "react";
import { commands } from "../bindings";

export function Connect() {
  const [ports, setPorts] = useState<string[]>([]);
  const [connecting, setConnecting] = useState(false);

  useEffect(() => {
    commands.getSerialPorts().then((ports) => {
      if (ports.status == "ok") {
        setPorts(ports.data);
      }
    });
  }, []);

  return (
    <div>
      <ul>
        {ports.map((port) => (
          <li key={port}>
            <Button
              disabled={connecting}
              onClick={async () => {
                setConnecting(true);
                const res = await commands.connect(port);
                if (res.status == "ok") {
                  notifications.show({
                    message: `Connected to ${port}`,
                  });
                } else {
                  notifications.show({
                    message: `Failed to connect to ${port} (${res.error})`,
                  });
                }
                setConnecting(false);
              }}
            >
              {port}
            </Button>
          </li>
        ))}
      </ul>
      <Button
        onClick={async () => {
          let res = await commands.disconnect();
          notifications.show({
            message: res.status == "ok"
              ? "Disconnected"
              : `Failed to disconnect (${res.error})`,
          });
        }}
      >
        Disconnect
      </Button>
    </div>
  );
}
