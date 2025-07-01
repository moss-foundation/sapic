import { EntryInfo } from "@repo/moss-collection";
import { useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamCollectionEntries";

export interface UseUpdateCollectionEntryInput {
  id: string;
  collectionId: string;
  updatedEntry: EntryInfo;
}

export const useUpdateCollectionEntry = () => {
  const queryClient = useQueryClient();

  const placeholderFnForUpdateCollectionEntry = ({ id, collectionId, updatedEntry }: UseUpdateCollectionEntryInput) => {
    queryClient.setQueryData([USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, collectionId], (old: EntryInfo[]) => {
      return old.map((c) => (c.id === id ? updatedEntry : c));
    });
  };

  return {
    placeholderFnForUpdateCollectionEntry,
  };
};
