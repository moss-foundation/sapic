import { useMemo } from "react";

import { EntryInfo } from "@repo/moss-collection";
import { StreamCollectionsEvent } from "@repo/moss-workspace";
import { useQueries } from "@tanstack/react-query";

import { fetchCollectionEntries } from "../queries/fetchCollectionEntries";
import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "../useStreamedCollectionEntries";
import { useStreamedCollections } from "../useStreamedCollections";

export interface CollectionWithEntries extends StreamCollectionsEvent {
  entries: EntryInfo[];
  isEntriesLoading: boolean;
  entriesError?: Error | null;
}

export const useStreamedCollectionsWithEntries = () => {
  const { data: collections = [], isLoading: isCollectionsLoading, error: collectionsError } = useStreamedCollections();

  const entriesQueries = useQueries({
    queries: collections.map((collection) => ({
      queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collection.id],
      queryFn: () => fetchCollectionEntries(collection.id),
      placeholderData: [] as EntryInfo[],
    })),
    combine: (results) => {
      return {
        data: results.map((result) => result.data || []),
        isLoading: results.some((result) => result.isPending),
        hasError: results.some((result) => result.error),
        results: results,
      };
    },
  });

  const collectionsWithEntries = useMemo((): CollectionWithEntries[] => {
    return collections.map((collection, index) => {
      const entriesResult = entriesQueries.results[index];
      return {
        ...collection,
        entries: entriesResult?.data || [],
        isEntriesLoading: entriesResult?.isPending || false,
        entriesError: entriesResult?.error || null,
      };
    });
  }, [collections, entriesQueries.results]);

  return {
    data: collectionsWithEntries,
    isLoading: isCollectionsLoading,
    error: collectionsError,
    isEntriesLoading: entriesQueries.isLoading,
    hasEntriesError: entriesQueries.hasError,
  };
};
