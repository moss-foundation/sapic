import { useMemo } from "react";

import { EntryInfo } from "@repo/moss-collection";
import { StreamCollectionsEvent } from "@repo/moss-workspace";
import { useQueries } from "@tanstack/react-query";

import { fetchCollectionEntries } from "./queries/fetchCollectionEntries";
import { USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamCollectionEntries";
import { useStreamedCollections } from "./useStreamedCollections";

export interface CollectionWithEntries extends StreamCollectionsEvent {
  entries: EntryInfo[];
  isEntriesLoading: boolean;
  entriesError?: Error | null;
}

export const useStreamedCollectionsWithEntries = () => {
  const { data: collections = [], isLoading: isCollectionsLoading, error: collectionsError } = useStreamedCollections();

  const entriesQueries = useQueries({
    queries: collections.map((collection) => ({
      queryKey: [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, collection.id],
      queryFn: () => fetchCollectionEntries(collection.id),
      placeholderData: [] as EntryInfo[],
    })),
  });

  const collectionsWithEntries = useMemo((): CollectionWithEntries[] => {
    return collections.map((collection, index) => {
      const entriesQuery = entriesQueries[index];
      return {
        ...collection,
        entries: entriesQuery?.data || [],
        isEntriesLoading: entriesQuery?.isLoading || false,
        entriesError: entriesQuery?.error || null,
      };
    });
  }, [collections, entriesQueries]);

  return {
    data: collectionsWithEntries,
    isLoading: isCollectionsLoading,
    error: collectionsError,
    isEntriesLoading: entriesQueries.some((query) => query.isLoading),
    hasEntriesError: entriesQueries.some((query) => query.error),
  };
};
