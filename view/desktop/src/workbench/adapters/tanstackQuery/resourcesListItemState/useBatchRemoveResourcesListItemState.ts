import { resourcesListItemStateService } from "@/workbench/services/resourcesListItemStateService";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY } from "../..";

export const USE_BATCH_REMOVE_RESOURCES_LIST_ITEM_STATE_MUTATION_KEY = "batchRemoveResourcesListItemState" as const;

export const useBatchRemoveResourcesListItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { ids: string[]; workspaceId: string }>({
    mutationKey: [USE_BATCH_REMOVE_RESOURCES_LIST_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ ids, workspaceId }) => resourcesListItemStateService.batchRemove(ids, workspaceId),
    onSuccess: (_, { ids, workspaceId }) => {
      ids.forEach((id) => {
        queryClient.removeQueries({ queryKey: [USE_GET_RESOURCES_LIST_ITEM_STATE_QUERY_KEY, id, workspaceId] });
      });
    },
  });
};
