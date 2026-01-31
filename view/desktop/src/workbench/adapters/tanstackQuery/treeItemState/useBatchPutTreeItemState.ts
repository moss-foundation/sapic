import { treeItemStateService } from "@/workbench/domains/treeItemState/service";
import { TreeItemState } from "@/workbench/domains/treeItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_BATCH_GET_TREE_ITEM_STATE_QUERY_KEY } from "./useBatchGetTreeItemState";
import { USE_GET_TREE_ITEM_STATE_QUERY_KEY } from "./useGetTreeItemState";

export const USE_BATCH_PUT_TREE_ITEM_STATE_MUTATION_KEY = "batchPutTreeItemState";

export const useBatchPutTreeItemState = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, { treeItemStates: TreeItemState[]; workspaceId: string }>({
    mutationKey: [USE_BATCH_PUT_TREE_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ treeItemStates, workspaceId }) => treeItemStateService.batchPut(treeItemStates, workspaceId),
    onSuccess: (_, { treeItemStates, workspaceId }) => {
      queryClient.invalidateQueries({ queryKey: [USE_BATCH_GET_TREE_ITEM_STATE_QUERY_KEY] });
      treeItemStates.forEach((treeItemState) => {
        queryClient.setQueryData([USE_GET_TREE_ITEM_STATE_QUERY_KEY, treeItemState.id, workspaceId], treeItemState);
      });
    },
  });
};
