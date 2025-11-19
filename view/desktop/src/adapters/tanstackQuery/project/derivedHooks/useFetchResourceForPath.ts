import { StreamResourcesEvent } from "@repo/moss-project";
import { useQueryClient } from "@tanstack/react-query";

import { startStreamingProjectResources } from "../queries/startStreamingProjectResources";
import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "../useStreamProjectResources";

export const useFetchResourcesForPath = () => {
  const queryClient = useQueryClient();

  const fetchResourcesForPath = async (projectId: string, path: string): Promise<StreamResourcesEvent[]> => {
    try {
      const newResources = await startStreamingProjectResources(projectId, path);

      queryClient.setQueryData<StreamResourcesEvent[]>(
        [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, projectId],
        (oldResources) => {
          if (!oldResources) return newResources;

          const newResourcesMap = new Map(newResources.map((resource) => [resource.id, resource]));

          const oldResourcesNotUpdated = oldResources.filter((resource) => !newResourcesMap.has(resource.id));

          return [...oldResourcesNotUpdated, ...newResources];
        }
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
