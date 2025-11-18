import { StreamResourcesEvent } from "@repo/moss-project";
import { useQuery, useQueryClient } from "@tanstack/react-query";

import { startStreamingProjectResources } from "./queries/startStreamingProjectResources";

export const USE_STREAM_PROJECT_RESOURCES_QUERY_KEY = "streamProjectResources";

export const useStreamProjectResources = (projectId: string) => {
  const queryClient = useQueryClient();

  const query = useQuery<StreamResourcesEvent[], Error>({
    queryKey: [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, projectId],
    queryFn: () => startStreamingProjectResources(projectId),
    placeholderData: [],
  });

  const clearResourcesCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, projectId] });
  };

  return {
    ...query,
    clearResourcesCacheAndRefetch,
  };
};
