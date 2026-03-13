import { useGetResourcesSummariesByProjectId } from "@/db/resourceSummaries/hooks/useGetResourcesSummariesByProjectId";
import { useCurrentWorkspace } from "@/hooks";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

export const useToggleAllTreeNodes = (id: string) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: localResourceSummaries } = useGetResourcesSummariesByProjectId(id);

  const expandAllNodes = async () => {
    const resourcesToUpdate = localResourceSummaries.filter(
      (resource) => !resource.expanded && resource.kind === "Dir"
    );

    if (resourcesToUpdate.length === 1) {
      await treeItemStateService.putExpanded(resourcesToUpdate[0].id, true, currentWorkspaceId);
    } else {
      await treeItemStateService.batchPutExpanded(
        Object.fromEntries(resourcesToUpdate.map((resource) => [resource.id, true])),
        currentWorkspaceId
      );
    }
  };

  const collapseAllNodes = async () => {
    const resourcesToUpdate = localResourceSummaries.filter((resource) => resource.expanded && resource.kind === "Dir");

    if (resourcesToUpdate.length === 1) {
      await treeItemStateService.putExpanded(resourcesToUpdate[0].id, false, currentWorkspaceId);
    } else {
      await treeItemStateService.batchPutExpanded(
        Object.fromEntries(resourcesToUpdate.map((resource) => [resource.id, false])),
        currentWorkspaceId
      );
    }
  };

  return { expandAllNodes, collapseAllNodes };
};
