import { treeItemStateService } from "@/workbench/domains/treeItemState/service";
import { TreeItemState } from "@/workbench/domains/treeItemState/types";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_TREE_ITEM_STATE_QUERY_KEY = "getTreeItemState";

export const useGetTreeItemState = (id: string, workspaceId: string) => {
  return useQuery<TreeItemState, Error>({
    queryKey: [USE_GET_TREE_ITEM_STATE_QUERY_KEY, id, workspaceId],
    queryFn: () => treeItemStateService.get(id, workspaceId),
  });
};
