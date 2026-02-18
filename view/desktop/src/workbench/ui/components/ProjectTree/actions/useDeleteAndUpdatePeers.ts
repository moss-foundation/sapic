import { useDeleteProjectResource } from "@/adapters";
import { useBatchUpdateProjectResource } from "@/adapters/tanstackQuery/resource/useBatchUpdateProjectResource";
import { useCurrentWorkspace } from "@/hooks";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { ProjectTreeNode, ProjectTreeRootNode } from "../types";
import { getAllNestedResources, siblingsAfterRemovalPayload } from "../utils";

export const useDeleteAndUpdatePeers = (
  projectId: string,
  node: ProjectTreeNode,
  parentNode: ProjectTreeNode | ProjectTreeRootNode
) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutateAsync: deleteProjectResource } = useDeleteProjectResource();
  const { mutateAsync: batchUpdateProjectResource } = useBatchUpdateProjectResource();

  const deleteAndUpdatePeers = async () => {
    await deleteProjectResource({
      projectId,
      input: {
        id: node.id,
      },
    });

    const allNestedChildren = getAllNestedResources(node);

    await treeItemStateService.batchRemoveOrder(
      allNestedChildren.map((child) => child.id),
      currentWorkspaceId
    );

    const sortedChildren = sortObjectsByOrder(parentNode.childNodes);
    const index = sortedChildren.findIndex((e) => e.id === node.id) + 1;
    const updatedParentNodeChildren = sortedChildren.slice(index).map((e) => ({
      ...e,
      order: e.order! - 1,
    }));

    await batchUpdateProjectResource({
      projectId,
      resources: {
        resources: siblingsAfterRemovalPayload({
          nodes: parentNode.childNodes,
          removedNode: node,
        }),
      },
    });

    await treeItemStateService.batchPutOrder(
      Object.fromEntries(updatedParentNodeChildren.map((child) => [child.id, child.order! - 1])),
      currentWorkspaceId
    );

    await treeItemStateService.removeOrder(node.id, currentWorkspaceId);
  };

  return {
    deleteAndUpdatePeers,
  };
};
