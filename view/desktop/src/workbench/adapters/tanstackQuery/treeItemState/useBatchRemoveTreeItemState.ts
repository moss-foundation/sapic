import { treeItemStateService } from "@/workbench/domains/treeItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_TREE_ITEM_STATE_QUERY_KEY } from "./useBatchGetTreeItemState";
import { USE_GET_TREE_ITEM_STATE_QUERY_KEY } from "./useGetTreeItemState";

export const USE_BATCH_REMOVE_TREE_ITEM_STATE_MUTATION_KEY = "batchRemoveTreeItemState";

export const useBatchRemoveTreeItemState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { ids: string[]; workspaceId: string }>({
    mutationKey: [USE_BATCH_REMOVE_TREE_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ ids, workspaceId }) => treeItemStateService.batchRemove(ids, workspaceId),
    onSuccess: (_, { ids, workspaceId }) => {
      ids.forEach((id) => {
        queryClient.removeQueries({ queryKey: [USE_GET_TREE_ITEM_STATE_QUERY_KEY, id, workspaceId] });
      });

      queryClient.setQueryData([USE_BATCH_GET_TREE_ITEM_STATE_QUERY_KEY, ids, workspaceId], []);
    },
  });
};
