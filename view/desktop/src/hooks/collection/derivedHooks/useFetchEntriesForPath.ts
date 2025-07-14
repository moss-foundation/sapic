import { EntryInfo } from "@repo/moss-collection";
import { useQueryClient } from "@tanstack/react-query";

import { fetchCollectionEntries } from "../queries/fetchCollectionEntries";
import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "../useStreamedCollectionEntries";

export const useFetchEntriesForPath = () => {
  const queryClient = useQueryClient();

  const fetchEntriesForPath = async (collectionId: string, path: string): Promise<EntryInfo[]> => {
    try {
      const newEntries = await fetchCollectionEntries(collectionId, path);

      queryClient.setQueryData<EntryInfo[]>([USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collectionId], (oldEntries) => {
        if (!oldEntries) return newEntries;

        const normalizedPath = path.replace(/\/$/, "");

        const entriesNotInPath = oldEntries.filter((entry) => {
          const entryPath = entry.path.raw.replace(/\/$/, "");
          return !entryPath.startsWith(normalizedPath);
        });

        return [...entriesNotInPath, ...newEntries];
      });

      return newEntries;
    } catch (error) {
      console.error(`Failed to fetch entries for path ${path}:`, error);
      throw error;
    }
  };

  return {
    fetchEntriesForPath,
  };
};
