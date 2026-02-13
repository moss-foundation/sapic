import { useGetResourcesSummariesByProjectId } from "@/db/resourceSummaries/hooks/useGetResourcesSummariesByProjectId";
import { useCurrentWorkspace } from "@/hooks";
import { useBatchPutTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useBatchPutTreeItemState";
import { usePutTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/usePutTreeItemState";

export const useToggleAllTreeNodes = (id: string) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const localResourceSummaries = useGetResourcesSummariesByProjectId(id);

  const { mutateAsync: putTreeItemState } = usePutTreeItemState();
  const { mutateAsync: batchPutTreeItemState } = useBatchPutTreeItemState();

  const expandAllNodes = async () => {
    const resourcesToUpdate = localResourceSummaries.filter(
      (resource) => !resource.expanded && resource.kind === "Dir"
    );

    if (resourcesToUpdate.length === 1) {
      await putTreeItemState({
        treeItemState: {
          id: resourcesToUpdate[0].id,
          expanded: true,
          order: resourcesToUpdate[0].order ?? undefined,
        },
        workspaceId: currentWorkspaceId,
      });
    } else {
      await batchPutTreeItemState({
        treeItemStates: resourcesToUpdate.map((resource) => ({
          id: resource.id,
          expanded: true,
          order: resource.order ?? undefined,
        })),
        workspaceId: currentWorkspaceId,
      });
    }
  };

  const collapseAllNodes = async () => {
    const resourcesToUpdate = localResourceSummaries.filter((resource) => resource.expanded && resource.kind === "Dir");

    if (resourcesToUpdate.length === 1) {
      await putTreeItemState({
        treeItemState: {
          id: resourcesToUpdate[0].id,
          expanded: false,
          order: resourcesToUpdate[0].order ?? undefined,
        },
        workspaceId: currentWorkspaceId,
      });
    } else {
      await batchPutTreeItemState({
        treeItemStates: resourcesToUpdate.map((resource) => ({
          id: resource.id,
          expanded: false,
          order: resource.order ?? undefined,
        })),
        workspaceId: currentWorkspaceId,
      });
    }
  };

  return { expandAllNodes, collapseAllNodes };
};
