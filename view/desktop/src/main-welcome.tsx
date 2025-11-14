import "@/app/i18n";

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import "./assets/index.css";

import { QueryCache, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { createHashHistory, createRootRoute, createRoute, createRouter, RouterProvider } from "@tanstack/react-router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { type } from "@tauri-apps/plugin-os";

import NotFoundPage from "./pages/NotFoundPage";
import { WelcomePage } from "./pages/welcome";

const ENABLE_REACT_QUERY_DEVTOOLS = import.meta.env.MODE === "development";
const queryClient = new QueryClient({
  queryCache: new QueryCache({
    onError: (err, query) => {
      console.error("Query client error", { err, query });
    },
  }),
  defaultOptions: {
    queries: {
      retry: false,
      networkMode: "always",
      refetchOnWindowFocus: false,
      refetchOnReconnect: false,
      refetchOnMount: false,
    },
  },
});
const rootRoute = createRootRoute({ notFoundComponent: NotFoundPage });
const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: WelcomePage,
});

export const welcomeRouter = createRouter({
  routeTree: rootRoute.addChildren([indexRoute]),
  history: createHashHistory(),
});

// const Workbench = lazy(() => import("@/components/Workbench").then((module) => ({ default: module.Workbench })));
const rootElement = document.getElementById("root") as HTMLElement;
if (rootElement) {
  // Prevent window flickering on startup by only showing the window after the webview is ready
  getCurrentWindow()
    .show()
    .then(() =>
      createRoot(rootElement).render(
        <StrictMode>
          <QueryClientProvider client={queryClient}>
            {ENABLE_REACT_QUERY_DEVTOOLS && <ReactQueryDevtools initialIsOpen={false} buttonPosition="bottom-right" />}
            <RouterProvider router={welcomeRouter} />
          </QueryClientProvider>
        </StrictMode>
      )
    );

  document.querySelector("html")!.classList.add(type());
}
