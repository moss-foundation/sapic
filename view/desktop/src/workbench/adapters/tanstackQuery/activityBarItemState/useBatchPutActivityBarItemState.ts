import { activityBarItemStateService } from "@/workbench/domains/activityBarItemState/service";
import { ActivityBarItemState } from "@/workbench/domains/activityBarItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useCurrentWorkspace } from "@/hooks";
import { placeholderActivityBarFirstItems } from "@/workbench/ui/parts/ActivityBar/components/placeholder";
import { USE_BATCH_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY } from "./useBatchGetActivityBarItemState";
import { USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY } from "./useGetActivityBarItemState";

export const USE_BATCH_PUT_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY = "batchPutActivityBarItemState";

export const useBatchPutActivityBarItemState = () => {
  const queryClient = useQueryClient();

  const { currentWorkspaceId } = useCurrentWorkspace();

  return useMutation<void, Error, { activityBarItemStates: ActivityBarItemState[]; workspaceId: string }>({
    mutationKey: [USE_BATCH_PUT_ACTIVITY_BAR_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ activityBarItemStates, workspaceId }) =>
      activityBarItemStateService.batchPut(activityBarItemStates, workspaceId),
    onSuccess: (_, { activityBarItemStates, workspaceId }) => {
      activityBarItemStates.forEach((activityBarItemState) => {
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
              const activityBarItemState = activityBarItemStates.find(
                (activityBarItemState) => activityBarItemState.id === oldState.id
              );

              if (!activityBarItemState) return oldState;

              return {
                ...oldState,
                ...activityBarItemState,
              };
            });
          }
        );
      });
    },
  });
};
