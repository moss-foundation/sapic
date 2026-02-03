import { activityBarItemStateService } from "@/workbench/domains/activityBarItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY } from "./useGetActivityBarItemState";

export const USE_REMOVE_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY = "removeActivityBarItemState";

export const useRemoveActivityBarItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { id: string }>({
    mutationKey: [USE_REMOVE_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ id }) => activityBarItemStateService.remove(id),
    onSuccess: (_, { id }) => {
      queryClient.removeQueries({ queryKey: [USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY, id] });
    },
  });
};
