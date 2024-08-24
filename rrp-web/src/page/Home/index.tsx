import { KeymapPage } from "./Keymap";
import {
  Tab,
  TabList,
  Toast,
  ToastTitle,
  useToastController,
} from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { KeyboardFilled, OptionsFilled } from "@fluentui/react-icons";
import { KeyboardOptionsPage } from "./KeyboardOptions";
import { Connection, useDisconnect } from "@/lib/connection";

export function Home({ connection }: { connection: Connection }) {
  const [selectedTab, setSelectedTab] = useState<"keymap" | "option">("keymap");

  const { dispatchToast } = useToastController();
  const disconnect = useDisconnect();

  useEffect(() => {
    const handler = () => {
      disconnect.mutate(false);
      dispatchToast(
        <Toast>
          <ToastTitle>
            Serial closed. disconnecting...
          </ToastTitle>
        </Toast>,
        { intent: "warning" },
      );
    };
    connection.port.addEventListener("disconnect", handler);

    return () => {
      connection.port.removeEventListener("disconnect", handler);
    };
  }, []);

  let page;
  if (selectedTab === "keymap") {
    page = <KeymapPage connection={connection} />;
  } else if (selectedTab === "option") {
    page = <KeyboardOptionsPage connection={connection} />;
  } else {
    page = <div></div>;
  }

  return (
    <div className="flex flex-col h-full">
      <TabList
        className="flex-shrink-0"
        selectedValue={selectedTab}
        onTabSelect={(_, d) => setSelectedTab(d.value as any)}
      >
        <Tab value="keymap" icon={<KeyboardFilled />}>
          Keymap
        </Tab>
        <Tab value="option" icon={<OptionsFilled />}>
          Keyboard options
        </Tab>
      </TabList>
      <div className="overflow-auto flex-grow bg-gray-200 dark:bg-gray-900">
        {page}
      </div>
    </div>
  );
}
