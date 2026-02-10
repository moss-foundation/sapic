import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY } from "./useBatchGetEnvironmentItemState";
import { USE_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY } from "./useGetEnvironmentItemState";

export const USE_BATCH_REMOVE_ENVIRONMENT_ITEM_STATE_MUTATION_KEY = "batchRemoveEnvironmentItemState" as const;

export const useBatchRemoveEnvironmentItemState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { ids: string[]; workspaceId: string }>({
    mutationKey: [USE_BATCH_REMOVE_ENVIRONMENT_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ ids, workspaceId }) => environmentItemStateService.batchRemove(ids, workspaceId),
    onSuccess: (_, { ids, workspaceId }) => {
      ids.forEach((id) => {
        queryClient.removeQueries({ queryKey: [USE_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY, id, workspaceId] });
      });

      queryClient.setQueryData([USE_BATCH_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY, ids, workspaceId], []);
    },
  });
};
