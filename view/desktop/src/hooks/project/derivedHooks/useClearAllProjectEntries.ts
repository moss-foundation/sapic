import { useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECT_ENTRIES_QUERY_KEY } from "../useStreamProjectEntries";

export const useClearAllProjectEntries = () => {
  const queryClient = useQueryClient();

  const clearAllProjectEntriesCache = () => {
    queryClient.removeQueries({
      queryKey: [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY],
      exact: false,
    });

    queryClient.invalidateQueries({
      queryKey: [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY],
      exact: false,
    });
  };

  return {
    clearAllProjectEntriesCache,
  };
};
