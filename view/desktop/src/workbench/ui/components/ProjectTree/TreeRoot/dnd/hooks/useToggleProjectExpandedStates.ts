import { useGetResourcesSummariesByProjectId } from "@/db/resourceSummaries/hooks/useGetResourcesSummariesByProjectId";
import { useCurrentWorkspace } from "@/hooks";
import { usePutEnvironmentListItemState, usePutResourcesListItemState } from "@/workbench/adapters";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

export const useToggleProjectExpandedStates = (id: string) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: localResourceSummaries } = useGetResourcesSummariesByProjectId(id);

  const { mutateAsync: updateEnvironmentListItemState } = usePutEnvironmentListItemState();
  const { mutateAsync: updateResourcesListItemState } = usePutResourcesListItemState();

  const toggleDirNodes = async (expanded: boolean) => {
    const resourcesToUpdate = localResourceSummaries.filter(
      (resource) => resource.kind === "Dir" && resource.expanded !== expanded
    );

    if (resourcesToUpdate.length === 0) return;

    if (resourcesToUpdate.length === 1) {
      await treeItemStateService.putExpanded(resourcesToUpdate[0].id, expanded, currentWorkspaceId);
    } else {
      await treeItemStateService.batchPutExpanded(
        Object.fromEntries(resourcesToUpdate.map((resource) => [resource.id, expanded])),
        currentWorkspaceId
      );
    }
  };

  const expandAll = async () => {
    await Promise.all([
      toggleDirNodes(true),
      updateEnvironmentListItemState({ id, expanded: true, workspaceId: currentWorkspaceId }),
      updateResourcesListItemState({ projectId: id, expanded: true, workspaceId: currentWorkspaceId }),
    ]);
  };

  const collapseAll = async () => {
    await Promise.all([
      toggleDirNodes(false),
      updateEnvironmentListItemState({ id, expanded: false, workspaceId: currentWorkspaceId }),
      updateResourcesListItemState({ projectId: id, expanded: false, workspaceId: currentWorkspaceId }),
    ]);
  };

  return { expandAll, collapseAll };
};
