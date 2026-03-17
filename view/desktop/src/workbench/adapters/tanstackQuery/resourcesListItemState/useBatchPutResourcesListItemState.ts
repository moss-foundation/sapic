import { resourcesListItemStateService } from "@/workbench/services/resourcesListItemStateService";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY } from "./useBatchGetResourcesListItemState";
import { USE_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY } from "./useGetResourcesListItemState";

export const USE_BATCH_PUT_RESOURCES_LIST_ITEM_STATE_MUTATION_KEY = "batchPutResourcesListItemState" as const;

export const useBatchPutResourcesListItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { resourcesListItemStates: Record<string, boolean>; workspaceId: string }>({
    mutationKey: [USE_BATCH_PUT_RESOURCES_LIST_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ resourcesListItemStates, workspaceId }) =>
      resourcesListItemStateService.batchPut(resourcesListItemStates, workspaceId),
    onSuccess: (_, { resourcesListItemStates, workspaceId }) => {
      const ids = Object.keys(resourcesListItemStates);
      queryClient.setQueryData(
        [USE_BATCH_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY, ids, workspaceId],
        ids.map((id) => resourcesListItemStates[id] ?? false)
      );

      ids.forEach((id) => {
        queryClient.setQueryData(
          [USE_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY, id, workspaceId],
          resourcesListItemStates[id] ?? false
        );
      });
    },
  });
};
