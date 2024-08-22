import { Button } from "@fluentui/react-components";
import { useConnect } from "@/lib/connection";

export function Connect() {
  const connect = useConnect();

  return (
    <div className="flex flex-col justify-center items-center flex-grow h-full">
      <p className="text-lg p-2">
        Press the connect button and select the keyboard to connect.
      </p>
      <Button
        appearance="primary"
        size="large"
        onClick={() => connect.mutate()}
      >
        Connect
      </Button>
    </div>
  );
}
