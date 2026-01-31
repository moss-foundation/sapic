import { showNotification } from "@/lib/ui/Notification";
import { MutationCache, QueryCache, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";

const ENABLE_REACT_QUERY_DEVTOOLS = import.meta.env.MODE === "development";
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
      networkMode: "always",
      refetchOnWindowFocus: false,
      refetchOnReconnect: false,
      refetchOnMount: false,
    },
  },
  queryCache: new QueryCache({
    onError: (err, query) => {
      console.error("Query client error", { err, query });
    },
  }),

  mutationCache: new MutationCache({
    onError: (err, query) => {
      console.error("Mutation client error", { err, query });

      //TODO This catches and shows any error in the app, it's an excessive way to handle errors, but it will do for now.
      // Handle both string and Error object types
      const errorMessage = typeof err === "string" ? err : err?.message || String(err);
      const errorName = typeof err === "string" ? "Error" : err?.name || "Error";

      // Clean up the error message - replace newlines and multiple spaces with single spaces
      const cleanedMessage = errorMessage.replace(/\n/g, " ").replace(/\s+/g, " ").trim();

      showNotification({
        title: errorName,
        description: cleanedMessage,
        icon: "Failed",
      });
    },
  }),
});

export const TanstackQueryClientProvider = ({ children }: { children: React.ReactNode }) => {
  return (
    <QueryClientProvider client={queryClient}>
      {ENABLE_REACT_QUERY_DEVTOOLS && <ReactQueryDevtools initialIsOpen={false} buttonPosition="bottom-right" />}
      {children}
    </QueryClientProvider>
  );
};
