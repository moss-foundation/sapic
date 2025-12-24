import { resourceService } from "@/domains/resource/resourceService";
import { StreamResourcesEvent } from "@repo/moss-project";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

export const USE_STREAM_PROJECT_RESOURCES_QUERY_KEY = "streamProjectResources";

const queryFn = async (projectId: string) => {
  const resources: StreamResourcesEvent[] = [];
  const onProjectResourceEvent = new Channel<StreamResourcesEvent>();

  onProjectResourceEvent.onmessage = (projectResource) => {
    resources.push(projectResource);
  };

  await resourceService.stream(projectId, onProjectResourceEvent);

  return resources;
};

export const useStreamProjectResources = (projectId: string) => {
  const queryClient = useQueryClient();

  const query = useQuery<StreamResourcesEvent[], Error>({
    queryKey: [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, projectId],
    queryFn: () => queryFn(projectId),
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
