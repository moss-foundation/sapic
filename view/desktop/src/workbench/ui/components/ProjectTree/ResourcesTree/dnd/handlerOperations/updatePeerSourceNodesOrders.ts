import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { ResourceNode } from "../../types";

export const updatePeerSourceNodesOrders = async ({
  sourceNodes,
  deletedNode,
  workspaceId,
}: {
  sourceNodes: ResourceNode[];
  deletedNode: ResourceNode;
  workspaceId: string;
}) => {
  const updatedPeerNodes = sourceNodes
    .filter((node) => node.id !== deletedNode.id)
    .map((node, index) => ({
      ...node,
      order: index + 1,
    }));

  const nodesWithDifferentOrders = updatedPeerNodes.filter((node) => {
    const originalNode = sourceNodes.find((n) => n.id === node.id);
    return originalNode?.order !== node.order;
  });

  if (nodesWithDifferentOrders.length === 1) {
    await treeItemStateService.putOrder(nodesWithDifferentOrders[0].id, nodesWithDifferentOrders[0].order, workspaceId);
  }

  if (nodesWithDifferentOrders.length > 1) {
    await treeItemStateService.batchPutOrder(
      Object.fromEntries(nodesWithDifferentOrders.map((node) => [node.id, node.order])),
      workspaceId
    );
  }
};
