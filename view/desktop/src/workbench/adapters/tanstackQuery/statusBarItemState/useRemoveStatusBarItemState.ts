import { statusBarItemStateService } from "@/workbench/services/statusBarItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY } from "./useGetStatusBarItemState";

export const USE_REMOVE_STATUS_BAR_ITEM_STATE_MUTATION_KEY = "removeStatusBarItemState" as const;

export const useRemoveStatusBarItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { id: string }>({
    mutationKey: [USE_REMOVE_STATUS_BAR_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ id }) => statusBarItemStateService.remove(id),
    onSuccess: (_, { id }) => {
      queryClient.removeQueries({ queryKey: [USE_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY, id] });
    },
  });
};
