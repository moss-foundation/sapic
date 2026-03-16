import { resourcesListItemStateService } from "@/workbench/services/resourcesListItemStateService";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY = "getResourcesListState" as const;

export const useGetResourcesListItemState = (projectId: string, workspaceId: string) => {
  return useQuery<boolean, Error>({
    queryKey: [USE_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY, projectId, workspaceId],
    queryFn: () => resourcesListItemStateService.get(projectId, workspaceId),
  });
};
