import "@/app/i18n";
import "@/assets/index.css";

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { TanstackQueryClientProvider } from "@/app/providers/TanstackQueryClientProvider";
import { onboardingRouter } from "@/onboarding/router/router";
import { RouterProvider } from "@tanstack/react-router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { type } from "@tauri-apps/plugin-os";

const rootElement = document.getElementById("root") as HTMLElement;
if (rootElement) {
  // Prevent window flickering on startup by only showing the window after the webview is ready
  getCurrentWindow()
    .show()
    .then(() =>
      createRoot(rootElement).render(
        <StrictMode>
          <TanstackQueryClientProvider>
            <RouterProvider router={onboardingRouter} />
          </TanstackQueryClientProvider>
        </StrictMode>
      )
    );

  document.querySelector("html")!.classList.add(type());
}
