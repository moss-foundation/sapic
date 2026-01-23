import "@/app/i18n";
import "@/assets/index.css";

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { scan } from "react-scan"; // must be imported before React and React DOM

import { TanstackQueryClientProvider } from "@/app/providers/TanstackQueryClientProvider";
import { mainRouter } from "@/main/router/router";
import { RouterProvider } from "@tanstack/react-router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { type } from "@tauri-apps/plugin-os";

scan({
  enabled: import.meta.env.MODE === "development" && type() !== "linux", //for whatever reason react scan causes flickering black screen on linux. So we disable it on linux temporarily.
});

const rootElement = document.getElementById("root") as HTMLElement;
if (rootElement) {
  // Prevent window flickering on startup by only showing the window after the webview is ready
  getCurrentWindow()
    .show()
    .then(() =>
      createRoot(rootElement).render(
        <StrictMode>
          <TanstackQueryClientProvider>
            <RouterProvider router={mainRouter} />
          </TanstackQueryClientProvider>
        </StrictMode>
      )
    );

  document.querySelector("html")!.classList.add(type());
}
