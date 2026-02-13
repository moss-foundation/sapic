import { treeItemStateService } from "@/workbench/domains/treeItemState/service";
import { TreeItemState } from "@/workbench/domains/treeItemState/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_TREE_ITEM_STATE_QUERY_KEY } from "./useGetTreeItemState";

export const USE_UPDATE_TREE_ITEM_STATE_MUTATION_KEY = "updateTreeItemState";

export const usePutTreeItemState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { treeItemState: TreeItemState; workspaceId: string }>({
    mutationKey: [USE_UPDATE_TREE_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ treeItemState, workspaceId }) => treeItemStateService.put(treeItemState, workspaceId),
    onSuccess: (_, { treeItemState, workspaceId }) => {
      queryClient.setQueryData([USE_GET_TREE_ITEM_STATE_QUERY_KEY, treeItemState.id, workspaceId], treeItemState);
    },
  });
};
