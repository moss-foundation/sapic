import { resourceService } from "@/domains/resource/resourceService";
import { ListProjectResourcesOutput } from "@repo/ipc";
import { useQuery, useQueryClient } from "@tanstack/react-query";

export const USE_LIST_PROJECT_RESOURCES_QUERY_KEY = "listProjectResources";

export const useListProjectResources = (projectId: string) => {
  const queryClient = useQueryClient();

  const query = useQuery<ListProjectResourcesOutput, Error>({
    queryKey: [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, projectId],
    queryFn: () => resourceService.list({ projectId, mode: "LOAD_ROOT" }),
    placeholderData: { items: [] },
  });

  const clearResourcesCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, projectId] });
  };

  return {
    ...query,
    clearResourcesCacheAndRefetch,
  };
};
