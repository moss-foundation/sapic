import { resourcesListItemStateService } from "@/workbench/services/resourcesListItemStateService";
import { useQuery } from "@tanstack/react-query";

export const USE_BATCH_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY = "batchGetResourcesListItemState" as const;

export const useBatchGetResourcesListItemState = (projectIds: string[], workspaceId: string) => {
  return useQuery<boolean[], Error>({
    queryKey: [USE_BATCH_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY, projectIds, workspaceId],
    queryFn: () => resourcesListItemStateService.batchGet(projectIds, workspaceId),
  });
};
