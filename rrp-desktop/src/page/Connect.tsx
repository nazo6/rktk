import { commands } from "../bindings";
import { useQuery } from "@tanstack/react-query";
import { unwrapped } from "../utils";
import {
  Button,
  Caption1,
  Card,
  CardHeader,
  Title2,
  Tooltip,
} from "@fluentui/react-components";
import { ArrowSyncRegular } from "@fluentui/react-icons";
import { useConnect, useDisconnect } from "../lib/connectHook";
import { ConnectionInfo } from "@/App";

export function Connect(
  props: { setConnectionInfo: (info: ConnectionInfo) => void },
) {
  const { data: ports, error, refetch: refetchPorts } = useQuery({
    queryKey: ["getSerialPorts"],
    queryFn: unwrapped(commands.getSerialPorts),
  });

  const disconnect = useDisconnect();
  const connect = useConnect();

  return (
    <div className="flex flex-col justify-center items-center flex-grow">
      {ports && (
        <div className="flex flex-col max-w-96">
          <Title2 className="pb-2">Select port to connect</Title2>
          <div className="flex mb-4 gap-2">
            <Button
              className="w-full"
              onClick={() => disconnect.mutate(true)}
            >
              Force Disconnect
            </Button>
            <Tooltip content="Refresh" relationship="label">
              <Button
                icon={<ArrowSyncRegular />}
                onClick={() => refetchPorts()}
              />
            </Tooltip>
          </div>
          <ul className="flex flex-col gap-2">
            {ports.map((port) => (
              <li key={port.port_name}>
                <Card className="w-full">
                  <CardHeader
                    header={port.port_name}
                    action={
                      <Button
                        disabled={connect.isPending}
                        onClick={async () => {
                          const keyboardInfo = await connect.mutateAsync(
                            port.port_name,
                          );
                          props.setConnectionInfo({
                            port,
                            keyboard: keyboardInfo,
                          });
                        }}
                      >
                        Connect
                      </Button>
                    }
                    description={
                      <Caption1>
                        {(typeof port.port_type != "string" &&
                            "UsbPort" in port.port_type)
                          ? (
                            <>
                              {port.port_type.UsbPort.manufacturer}
                              {" - "}
                              {port.port_type.UsbPort.product}
                            </>
                          )
                          : port.port_type}
                      </Caption1>
                    }
                  />
                </Card>
              </li>
            ))}
          </ul>
        </div>
      )}
      {error && <div>{error.message}</div>}
    </div>
  );
}
