import React from "react";
import ReactDOM from "react-dom/client";
import { Providers } from "./App";

import "./index.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Providers />
  </React.StrictMode>,
);
