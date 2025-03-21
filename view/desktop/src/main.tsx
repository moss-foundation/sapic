import { StrictMode, Suspense } from "react";
import { createRoot } from "react-dom/client";

import App from "./App";

import "overlayscrollbars/overlayscrollbars.css";
import "./assets/index.css";

import { QueryCache, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";

import GeneralProvider from "./app/Provider";

const ENABLE_REACT_QUERY_DEVTOOLS = true;

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

createRoot(document.getElementById("root") as HTMLElement).render(
  //<StrictMode>
  <QueryClientProvider client={queryClient}>
    {ENABLE_REACT_QUERY_DEVTOOLS && <ReactQueryDevtools initialIsOpen={false} buttonPosition="bottom-right" />}
    <GeneralProvider>
      <Suspense fallback={<div />}>
        <App />
      </Suspense>
    </GeneralProvider>
  </QueryClientProvider>
  // </StrictMode>
);
