import { lazy, StrictMode, Suspense } from "react";
import { createRoot } from "react-dom/client";

import "@/app/i18n";

import { PageLoader } from "./components/PageLoader";

import "allotment/dist/style.css";
import "overlayscrollbars/overlayscrollbars.css";
import "./assets/index.css";

import { QueryCache, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { type } from "@tauri-apps/plugin-os";

import GeneralProvider from "./app/Provider";

const ENABLE_REACT_QUERY_DEVTOOLS = import.meta.env.MODE === "development";

const queryClient = new QueryClient({
  queryCache: new QueryCache({
    onError: (err, query) => {
      console.log("Query client error", { err, query });
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

if (import.meta.env.MODE === "development") {
  const script = document.createElement("script");
  script.src = "http://localhost:8097";
  document.head.appendChild(script);
}

const App = lazy(() => import("@/app")); // lazy load the main App component
const rootElement = document.getElementById("root") as HTMLElement; // cache the root element reference

if (rootElement) {
  createRoot(rootElement).render(
    <StrictMode>
      <QueryClientProvider client={queryClient}>
        {ENABLE_REACT_QUERY_DEVTOOLS && <ReactQueryDevtools initialIsOpen={false} buttonPosition="bottom-right" />}
        <GeneralProvider>
          <Suspense fallback={<PageLoader />}>
            <App />
          </Suspense>
        </GeneralProvider>
      </QueryClientProvider>
    </StrictMode>
  );

  document.querySelector("html")!.classList.add(type());
}
