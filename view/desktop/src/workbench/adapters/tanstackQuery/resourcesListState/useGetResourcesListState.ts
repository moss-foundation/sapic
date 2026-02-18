import { resourcesListItemStateService } from "@/workbench/services/resourcesListItemStateService";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_RESOURCES_LIST_STATE_QUERY_KEY = "getResourcesListState" as const;

export const useGetResourcesListState = (resourcesListItemId: string, workspaceId: string) => {
  return useQuery<boolean, Error>({
    queryKey: [USE_GET_RESOURCES_LIST_STATE_QUERY_KEY, resourcesListItemId, workspaceId],
    queryFn: () => resourcesListItemStateService.get(resourcesListItemId, workspaceId),
  });
};
