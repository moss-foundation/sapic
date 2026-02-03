import { activityBarItemStateService } from "@/workbench/domains/activityBarItemState/service";
import { ActivityBarItemState } from "@/workbench/domains/activityBarItemState/types";
import { placeholderActivityBarFirstItems } from "@/workbench/ui/parts/ActivityBar/components/placeholder";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY } from "./useBatchGetActivityBarItemState";
import { USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY } from "./useGetActivityBarItemState";

export const USE_PUT_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY = "putActivityBarItemState";

export const usePutActivityBarItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { activityBarItemState: ActivityBarItemState }>({
    mutationKey: [USE_PUT_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ activityBarItemState }) => activityBarItemStateService.put(activityBarItemState),
    onSuccess: (_, { activityBarItemState }) => {
      queryClient.setQueryData(
        [USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY, activityBarItemState.id],
        activityBarItemState
      );

      //update the specific cache that contains all first items
      const firstItemsIds = placeholderActivityBarFirstItems.map((item) => item.id);
      queryClient.setQueryData(
        [USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY, firstItemsIds],
        (oldData: ActivityBarItemState[]) => {
          return oldData.map((oldState) => {
            if (oldState.id === activityBarItemState.id)
              return {
                ...oldState,
                ...activityBarItemState,
              };

            return oldState;
          });
        }
      );
    },
  });
};
