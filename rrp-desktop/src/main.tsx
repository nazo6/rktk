import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import { MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";

import "./index.css";
import "@mantine/core/styles.css";
import "@mantine/notifications/styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <MantineProvider>
      <Notifications />
      <App />
    </MantineProvider>
  </React.StrictMode>,
);
