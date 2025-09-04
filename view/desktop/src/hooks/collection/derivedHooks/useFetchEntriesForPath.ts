import { StreamEntriesEvent } from "@repo/moss-collection";
import { useQueryClient } from "@tanstack/react-query";

import { startStreamingCollectionEntries } from "../queries/startStreamingCollectionEntries";
import { USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY } from "../useStreamCollectionEntries";

export const useFetchEntriesForPath = () => {
  const queryClient = useQueryClient();

  const fetchEntriesForPath = async (collectionId: string, path: string): Promise<StreamEntriesEvent[]> => {
    try {
      const newEntries = await startStreamingCollectionEntries(collectionId, path);

      queryClient.setQueryData<StreamEntriesEvent[]>(
        [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, collectionId],
        (oldEntries) => {
          if (!oldEntries) return newEntries;

          const newEntriesMap = new Map(newEntries.map((entry) => [entry.id, entry]));

          const oldEntriesNotUpdated = oldEntries.filter((entry) => !newEntriesMap.has(entry.id));

          return [...oldEntriesNotUpdated, ...newEntries];
        }
      );

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
