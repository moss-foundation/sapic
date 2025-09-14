import { StreamEntriesEvent } from "@repo/moss-collection";
import { useQuery, useQueryClient } from "@tanstack/react-query";

import { useActiveWorkspace } from "../workspace/derived/useActiveWorkspace";
import { startStreamingCollectionEntries } from "./queries/startStreamingCollectionEntries";

export const USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY = "streamCollectionEntries";

export const useStreamCollectionEntries = (collectionId: string) => {
  const queryClient = useQueryClient();

  const { hasActiveWorkspace } = useActiveWorkspace();

  const query = useQuery<StreamEntriesEvent[], Error>({
    queryKey: [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, collectionId],
    queryFn: async () => {
      const entires = await startStreamingCollectionEntries(collectionId);
      return entires;
    },
    placeholderData: [],
    enabled: hasActiveWorkspace,
  });

  const clearEntriesCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY] });
  };

  return {
    ...query,
    clearEntriesCacheAndRefetch,
  };
};
