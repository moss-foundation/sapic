import { useDeleteProjectResource } from "@/adapters";
import { useBatchUpdateProjectResource } from "@/adapters/tanstackQuery/resource/useBatchUpdateProjectResource";
import { useCurrentWorkspace } from "@/hooks";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { ResourceNode, ResourcesTree } from "../../types";
import { getAllNestedResources, siblingsAfterRemovalPayload } from "../../utils";

interface UseDeleteAndUpdateResourceNodePeersProps {
  projectId: string;
  node: ResourceNode;
  parentNode: ResourceNode | ResourcesTree;
}

//TODO finish this hook
export const useDeleteAndUpdateResourceNodePeers = ({
  projectId,
  node,
  parentNode,
}: UseDeleteAndUpdateResourceNodePeersProps) => {
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

    //TODO make another way to check if the parentNode is a resources list root
    const isResourcesListRoot = "projectId" in parentNode;

    const peerNodes = isResourcesListRoot ? node.childNodes : parentNode.childNodes;

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
