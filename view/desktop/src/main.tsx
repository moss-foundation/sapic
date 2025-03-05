import React from "react";
import { createRoot } from "react-dom/client";

import App from "./App";

import "overlayscrollbars/overlayscrollbars.css";
import "./assets/index.css";

if (import.meta.env.MODE === "development") {
  const script = document.createElement("script");
  script.src = "http://localhost:8097";
  document.head.appendChild(script);
}

createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
