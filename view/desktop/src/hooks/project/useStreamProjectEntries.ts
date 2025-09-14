import { StreamEntriesEvent } from "@repo/moss-project";
import { useQuery, useQueryClient } from "@tanstack/react-query";

import { useActiveWorkspace } from "../workspace/derived/useActiveWorkspace";
import { startStreamingProjectEntries } from "./queries/startStreamingProjectEntries";

export const USE_STREAM_PROJECT_ENTRIES_QUERY_KEY = "streamProjectEntries";

export const useStreamProjectEntries = (projectId: string) => {
  const queryClient = useQueryClient();

  const { hasActiveWorkspace } = useActiveWorkspace();

  const query = useQuery<StreamEntriesEvent[], Error>({
    queryKey: [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, projectId],
    queryFn: async () => {
      const entires = await startStreamingProjectEntries(projectId);
      return entires;
    },
    placeholderData: [],
    enabled: hasActiveWorkspace,
  });

  const clearEntriesCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY] });
  };

  return {
    ...query,
    clearEntriesCacheAndRefetch,
  };
};
