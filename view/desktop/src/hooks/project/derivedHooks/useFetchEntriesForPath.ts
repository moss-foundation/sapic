import { StreamResourcesEvent } from "@repo/moss-project";
import { useQueryClient } from "@tanstack/react-query";

import { startStreamingProjectEntries } from "../queries/startStreamingProjectEntries";
import { USE_STREAM_PROJECT_ENTRIES_QUERY_KEY } from "../useStreamProjectEntries";

export const useFetchEntriesForPath = () => {
  const queryClient = useQueryClient();

  const fetchEntriesForPath = async (projectId: string, path: string): Promise<StreamResourcesEvent[]> => {
    try {
      const newEntries = await startStreamingProjectEntries(projectId, path);

      queryClient.setQueryData<StreamResourcesEvent[]>(
        [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, projectId],
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
