import { activityBarItemStateService } from "@/workbench/domains/activityBarItemState/service";
import { ActivityBarItemState } from "@/workbench/domains/activityBarItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useCurrentWorkspace } from "@/hooks";
import { placeholderActivityBarFirstItems } from "@/workbench/ui/parts/ActivityBar/placeholder";
import { USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY } from "./useBatchGetActivityBarItemState";
import { USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY } from "./useGetActivityBarItemState";

export const USE_PUT_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY = "putActivityBarItemState";

export const usePutActivityBarItemState = () => {
  const queryClient = useQueryClient();

  const { currentWorkspaceId } = useCurrentWorkspace();

  return useMutation<void, Error, { activityBarItemState: ActivityBarItemState; workspaceId: string }>({
    mutationKey: [USE_PUT_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ activityBarItemState, workspaceId }) =>
      activityBarItemStateService.put(activityBarItemState, workspaceId),
    onSuccess: (_, { activityBarItemState, workspaceId }) => {
      queryClient.setQueryData(
        [USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY, activityBarItemState.id, workspaceId],
        activityBarItemState
      );

      //update the specific cache that contains all first items
      const firstItemsIds = placeholderActivityBarFirstItems.map((item) => item.id);
      queryClient.setQueryData(
        [USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY, firstItemsIds, currentWorkspaceId],
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
