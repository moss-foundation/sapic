import { USE_LIST_PROJECT_RESOURCES_QUERY_KEY } from "@/adapters/tanstackQuery/resource/useListProjectResources";
import { resourceService } from "@/domains/resource/resourceService";
import { useQueryClient } from "@tanstack/react-query";

export const useRefreshProject = (projectId: string) => {
  const queryClient = useQueryClient();

  const refreshProject = async () => {
    queryClient.invalidateQueries({
      queryKey: [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, projectId],
    });
    queryClient.removeQueries({ queryKey: [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, projectId] });

    await resourceService.list({ projectId, mode: { "RELOAD_PATH": "" } });
  };

  return { refreshProject };
};
