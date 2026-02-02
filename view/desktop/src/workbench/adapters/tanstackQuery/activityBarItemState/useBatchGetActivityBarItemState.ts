import { activityBarItemStateService } from "@/workbench/domains/activityBarItemState/service";
import { ActivityBarItemState } from "@/workbench/domains/activityBarItemState/types";
import { useQuery } from "@tanstack/react-query";

export const USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY = "batchGetActivityBarItemState";

export const useBatchGetActivityBarItemState = (ids: string[], workspaceId: string) => {
  return useQuery<ActivityBarItemState[], Error>({
    queryKey: [USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY, ids, workspaceId],
    queryFn: () => activityBarItemStateService.batchGet(ids, workspaceId),
  });
};
