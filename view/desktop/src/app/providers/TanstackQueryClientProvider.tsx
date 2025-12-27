import { toast } from "sonner";

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
      toast.error(err.message);
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
