import { statusBarItemStateService } from "@/workbench/services/statusBarItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY } from "./useBatchGetStatusBarItemState";
import { USE_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY } from "./useGetStatusBarItemState";

export const USE_BATCH_REMOVE_STATUS_BAR_ITEM_STATE_MUTATION_KEY = "batchRemoveStatusBarItemState" as const;

export const useBatchRemoveStatusBarItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { ids: string[] }>({
    mutationKey: [USE_BATCH_REMOVE_STATUS_BAR_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ ids }) => statusBarItemStateService.batchRemove(ids),
    onSuccess: (_, { ids }) => {
      ids.forEach((id) => {
        queryClient.removeQueries({ queryKey: [USE_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY, id] });
      });
      queryClient.removeQueries({ queryKey: [USE_BATCH_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY, ids] });
    },
  });
};
