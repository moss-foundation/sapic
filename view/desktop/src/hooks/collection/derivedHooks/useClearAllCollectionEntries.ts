import { useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY } from "../useStreamCollectionEntries";

export const useClearAllCollectionEntries = () => {
  const queryClient = useQueryClient();

  const clearAllCollectionEntriesCache = () => {
    queryClient.removeQueries({
      queryKey: [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY],
      exact: false,
    });

    queryClient.invalidateQueries({
      queryKey: [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY],
      exact: false,
    });
  };

  return {
    clearAllCollectionEntriesCache,
  };
};
