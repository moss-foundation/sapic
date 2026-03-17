import { environmentListItemStateService } from "@/workbench/services/environmentListItemStateService";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY } from "./useBatchGetEnvironmentListItemState";
import { USE_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY } from "./useGetEnvironmentListItemState";

export const USE_BATCH_PUT_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY = "batchPutEnvironmentListItemState" as const;

export const useBatchPutEnvironmentListItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { environmentListItemStates: Record<string, boolean>; workspaceId: string }>({
    mutationKey: [USE_BATCH_PUT_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ environmentListItemStates, workspaceId }) =>
      environmentListItemStateService.batchPutExpanded(environmentListItemStates, workspaceId),
    onSuccess: (_, { environmentListItemStates, workspaceId }) => {
      const ids = Object.keys(environmentListItemStates);
      queryClient.setQueryData(
        [USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, ids, workspaceId],
        ids.map((id) => environmentListItemStates[id] ?? false)
      );

      ids.forEach((id) => {
        queryClient.setQueryData(
          [USE_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, id, workspaceId],
          environmentListItemStates[id] ?? false
        );
      });
    },
  });
};
