import { statusBarItemStateService } from "@/workbench/services/statusBarItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY } from "./useBatchGetStatusBarItemState";

export const USE_BATCH_PUT_STATUS_BAR_ITEM_STATE_MUTATION_KEY = "batchPutStatusBarItemState" as const;

export const useBatchPutStatusBarItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { statusBarItemStates: Record<string, number> }>({
    mutationKey: [USE_BATCH_PUT_STATUS_BAR_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ statusBarItemStates }) => statusBarItemStateService.batchPut(statusBarItemStates),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_BATCH_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY] });
    },
  });
};
