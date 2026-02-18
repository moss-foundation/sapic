import { environmentListItemStateService } from "@/workbench/services/environmentListItemStateService";
import { useQuery } from "@tanstack/react-query";

export const USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY = "batchGetEnvironmentListItemState" as const;

export const useBatchGetEnvironmentListItemState = (ids: string[], workspaceId: string) => {
  return useQuery<boolean[], Error>({
    queryKey: [USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, ids, workspaceId],
    queryFn: () => environmentListItemStateService.batchGetExpanded(ids, workspaceId),
  });
};
