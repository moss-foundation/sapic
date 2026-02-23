import { activityBarItemStateService } from "@/workbench/services/activityBarItemState/service";
import { ActivityBarItemState } from "@/workbench/services/activityBarItemState/types";
import { useQuery } from "@tanstack/react-query";

export const USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY = "batchGetActivityBarItemState";

export const useBatchGetActivityBarItemState = (ids: string[]) => {
  return useQuery<ActivityBarItemState[], Error>({
    queryKey: [USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY, ids],
    queryFn: () => activityBarItemStateService.batchGet(ids),
  });
};
