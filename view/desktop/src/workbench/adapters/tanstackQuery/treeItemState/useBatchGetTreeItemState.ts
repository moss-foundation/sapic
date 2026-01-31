import { treeItemStateService } from "@/workbench/domains/treeItemState/service";
import { TreeItemState } from "@/workbench/domains/treeItemState/types";
import { useQuery } from "@tanstack/react-query";

export const USE_BATCH_GET_TREE_ITEM_STATE_QUERY_KEY = "batchGetTreeItemState";

export const useBatchGetTreeItemState = (ids: string[], workspaceId: string) => {
  return useQuery<TreeItemState[], Error>({
    queryKey: [USE_BATCH_GET_TREE_ITEM_STATE_QUERY_KEY, ids, workspaceId],
    queryFn: () => treeItemStateService.batchGet(ids, workspaceId),
  });
};
