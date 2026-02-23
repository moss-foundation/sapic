import { statusBarItemStateService } from "@/workbench/services/statusBarItemState/service";
import { useQuery } from "@tanstack/react-query";

export const USE_BATCH_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY = "batchGetStatusBarItemState" as const;

export const useBatchGetStatusBarItemState = (ids: string[]) => {
  return useQuery<number[], Error>({
    queryKey: [USE_BATCH_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY, ids],
    queryFn: () => statusBarItemStateService.batchGet(ids),
  });
};
