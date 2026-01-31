import { treeItemStateService } from "@/workbench/domains/treeItemState/service";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_GET_TREE_ITEM_STATE_QUERY_KEY } from "./useGetTreeItemState";

export const USE_REMOVE_TREE_ITEM_STATE_MUTATION_KEY = "removeTreeItemState";

export const useRemoveTreeItemState = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, { id: string; workspaceId: string }>({
    mutationKey: [USE_REMOVE_TREE_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ id, workspaceId }) => treeItemStateService.remove(id, workspaceId),
    onSuccess: (_, { id, workspaceId }) => {
      queryClient.invalidateQueries({ queryKey: [USE_GET_TREE_ITEM_STATE_QUERY_KEY, id, workspaceId] });
    },
  });
};
