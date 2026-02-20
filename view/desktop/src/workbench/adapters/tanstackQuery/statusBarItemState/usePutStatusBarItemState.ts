import { statusBarItemStateService } from "@/workbench/services/statusBarItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY } from "./useGetStatusBarItemState";

export const USE_PUT_STATUS_BAR_ITEM_STATE_MUTATION_KEY = "putStatusBarItemState" as const;

export const usePutStatusBarItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { id: string; order: number }>({
    mutationKey: [USE_PUT_STATUS_BAR_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ id, order }) => statusBarItemStateService.put(id, order),
    onSuccess: (_, { id, order }) => {
      queryClient.setQueryData([USE_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY, id], order);
    },
  });
};
