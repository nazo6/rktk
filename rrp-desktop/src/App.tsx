import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { commands } from "./bindings";

function App() {
  const [data, setData] = useState("");

  useEffect(() => {
    (async () => {
      const info = await commands.keyboardInfo();
      setData(JSON.stringify(info));
    })();
  });

  return (
    <div className="container">
      <p>{data}</p>
    </div>
  );
}

export default App;
