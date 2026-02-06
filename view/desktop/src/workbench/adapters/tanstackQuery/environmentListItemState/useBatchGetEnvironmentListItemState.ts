import { environmentListItemStateService } from "@/workbench/domains/environmentListItemState/service";
import { EnvironmentListItemState } from "@/workbench/domains/environmentListItemState/types";
import { useQuery } from "@tanstack/react-query";

export const USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY = "batchGetEnvironmentListItemState" as const;

export const useBatchGetEnvironmentListItemState = (ids: string[], workspaceId: string) => {
  return useQuery<EnvironmentListItemState[], Error>({
    queryKey: [USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, ids, workspaceId],
    queryFn: () => environmentListItemStateService.batchGet(ids, workspaceId),
  });
};
