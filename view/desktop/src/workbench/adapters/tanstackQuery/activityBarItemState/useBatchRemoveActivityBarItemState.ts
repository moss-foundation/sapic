import { activityBarItemStateService } from "@/workbench/domains/activityBarItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY } from "./useBatchGetActivityBarItemState";
import { USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY } from "./useGetActivityBarItemState";

export const USE_BATCH_REMOVE_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY = "batchRemoveActivityBarItemState";

export const useBatchRemoveActivityBarItemState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { ids: string[] }>({
    mutationKey: [USE_BATCH_REMOVE_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ ids }) => activityBarItemStateService.batchRemove(ids),
    onSuccess: (_, { ids }) => {
      ids.forEach((id) => {
        queryClient.removeQueries({ queryKey: [USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY, id] });
      });
      queryClient.removeQueries({ queryKey: [USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY, ids] });
    },
  });
};
