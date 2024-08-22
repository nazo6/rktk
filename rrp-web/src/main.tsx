import React from "react";
import ReactDOM from "react-dom/client";
import { QueryClientProvider } from "@tanstack/react-query";
import { FluentProvider, webLightTheme } from "@fluentui/react-components";

import App from "./App";

import "./index.css";
import { queryClient } from "./queryClient";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <FluentProvider theme={webLightTheme}>
        <div className="h-[100vh]">
          <App />
        </div>
      </FluentProvider>
    </QueryClientProvider>
  </React.StrictMode>,
);
