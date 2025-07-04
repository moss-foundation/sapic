import { EntryInfo } from "@repo/moss-collection";
import { useQuery, useQueryClient } from "@tanstack/react-query";

import { fetchCollectionEntries } from "./queries/fetchCollectionEntries";

export const USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY = "streamCollectionEntries";

export const useStreamedCollectionEntries = (collectionId: string) => {
  const queryClient = useQueryClient();

  const query = useQuery<EntryInfo[], Error>({
    queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collectionId],
    queryFn: () => fetchCollectionEntries(collectionId),
    placeholderData: [],
  });

  const clearEntriesCacheAndRefetch = () => {
    queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY] });
    queryClient.removeQueries({ queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY] });
    return query.refetch();
  };

  return {
    ...query,
    clearEntriesCacheAndRefetch,
  };
};
