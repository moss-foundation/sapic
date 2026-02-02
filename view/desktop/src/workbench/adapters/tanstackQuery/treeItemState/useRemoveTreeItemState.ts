import { treeItemStateService } from "@/workbench/domains/treeItemState/service";
import { useMutation } from "@tanstack/react-query";

export const USE_REMOVE_TREE_ITEM_STATE_MUTATION_KEY = "removeTreeItemState";

export const useRemoveTreeItemState = () => {
  return useMutation<void, Error, { id: string; workspaceId: string }>({
    mutationKey: [USE_REMOVE_TREE_ITEM_STATE_MUTATION_KEY],
    mutationFn: ({ id, workspaceId }) => treeItemStateService.remove(id, workspaceId),
  });
};
