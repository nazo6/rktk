import { useEffect, useState } from "react";
import { events } from "./bindings";
import { Home } from "./page/Home";
import { Connect } from "./page/Connect";

export default function App() {
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const h = events.connectionEvent.listen((e) => {
      setConnected(e.payload);
    });
  });

  return (
    connected ? <Home /> : <Connect />
  );
}
