import { environmentListItemStateService } from "@/workbench/domains/environmentListItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY } from "./useBatchGetEnvironmentListItemState";

export const USE_BATCH_REMOVE_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY = "batchRemoveEnvironmentListItemState" as const;

export const useBatchRemoveEnvironmentListItemState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { ids: string[]; workspaceId: string }>({
    mutationKey: [USE_BATCH_REMOVE_ENVIRONMENT_LIST_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ ids, workspaceId }) => environmentListItemStateService.batchRemove(ids, workspaceId),
    onSuccess: (_, { ids, workspaceId }) => {
      ids.forEach((id) => {
        queryClient.removeQueries({ queryKey: [USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, id, workspaceId] });
      });

      queryClient.setQueryData([USE_BATCH_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, ids, workspaceId], []);
    },
  });
};
