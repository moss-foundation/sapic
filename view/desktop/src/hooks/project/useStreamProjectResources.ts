import { StreamResourcesEvent } from "@repo/moss-project";
import { useQuery, useQueryClient } from "@tanstack/react-query";

import { useActiveWorkspace } from "../workspace/derived/useActiveWorkspace";
import { startStreamingProjectResources } from "./queries/startStreamingProjectResources";

export const USE_STREAM_PROJECT_RESOURCES_QUERY_KEY = "streamProjectResources";

export const useStreamProjectResources = (projectId: string) => {
  const queryClient = useQueryClient();

  const { hasActiveWorkspace } = useActiveWorkspace();

  const query = useQuery<StreamResourcesEvent[], Error>({
    queryKey: [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, projectId],
    queryFn: async () => {
      const resources = await startStreamingProjectResources(projectId);
      return resources;
    },
    placeholderData: [],
    enabled: hasActiveWorkspace,
  });

  const clearResourcesCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY] });
  };

  return {
    ...query,
    clearResourcesCacheAndRefetch,
  };
};
