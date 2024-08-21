import { KeyboardInfo } from "@/bindings";
import { KeymapPage } from "./Keymap";
import { Tab, TabList } from "@fluentui/react-components";
import { useState } from "react";
import { KeyboardFilled, OptionsFilled } from "@fluentui/react-icons";
import { KeyboardOptionsPage } from "./KeyboardOptions";

export function Home({ keyboardInfo }: { keyboardInfo: KeyboardInfo }) {
  const [selectedTab, setSelectedTab] = useState<"keymap" | "option">("keymap");

  let page;
  if (selectedTab === "keymap") {
    page = <KeymapPage keyboardInfo={keyboardInfo} />;
  } else if (selectedTab === "option") {
    page = <KeyboardOptionsPage keyboardInfo={keyboardInfo} />;
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
      <div className="overflow-auto flex-grow bg-gray-200">
        {page}
      </div>
    </div>
  );
}
