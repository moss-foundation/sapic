import { resourceService } from "@/domains/resource/resourceService";
import { ListProjectResourcesOutput } from "@repo/ipc";
import { useQueryClient } from "@tanstack/react-query";

import { USE_LIST_PROJECT_RESOURCES_QUERY_KEY } from "../../resource/useListProjectResources";

export const useFetchResourcesForPath = () => {
  const queryClient = useQueryClient();

  const fetchResourcesForPath = async (projectId: string, path: string): Promise<ListProjectResourcesOutput> => {
    try {
      const newResources = await resourceService.list({ projectId, mode: { "RELOAD_PATH": path } });

      queryClient.setQueryData<ListProjectResourcesOutput>(
        [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, projectId],
        newResources
      );

      return newResources;
    } catch (error) {
      console.error(`Failed to fetch resources for path ${path}:`, error);
      throw error;
    }
  };

  return {
    fetchResourcesForPath,
  };
};
