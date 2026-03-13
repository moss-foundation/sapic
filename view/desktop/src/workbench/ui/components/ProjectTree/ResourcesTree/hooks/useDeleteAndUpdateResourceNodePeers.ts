import { resourceService } from "@/domains/resource/resourceService";
import { useCurrentWorkspace } from "@/hooks";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { ResourcesTreeRoot } from "../../TreeRoot/types";
import { getAllNestedResources } from "../../utils";
import { ResourceNode } from "../types";

interface UseDeleteAndUpdateResourceNodePeersProps {
  projectId: string;
  node: ResourceNode;
  parentNode: ResourceNode | ResourcesTreeRoot;
}

export const useDeleteAndUpdateResourceNodePeers = ({
  projectId,
  node,
  parentNode,
}: UseDeleteAndUpdateResourceNodePeersProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const deleteAndUpdatePeers = async () => {
    await resourceService.delete(projectId, {
      id: node.id,
    });

    const allNestedChildren = getAllNestedResources(node);
    await treeItemStateService.batchRemoveOrder(
      allNestedChildren.map((child) => child.id),
      currentWorkspaceId
    );

    const sortedChildren = sortObjectsByOrder(parentNode.childNodes);
    const updatedPeerNodes = sortedChildren
      .filter((e) => e.id !== node.id)
      .map((e, index) => ({
        ...e,
        order: index + 1,
      }));

    await treeItemStateService.batchPutOrder(
      Object.fromEntries(updatedPeerNodes.map((child) => [child.id, child.order])),
      currentWorkspaceId
    );
    await treeItemStateService.removeOrder(node.id, currentWorkspaceId);
  };

  return {
    deleteAndUpdatePeers,
  };
};
