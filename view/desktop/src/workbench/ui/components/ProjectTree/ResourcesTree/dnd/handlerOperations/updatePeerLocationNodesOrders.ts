import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { ResourceNode } from "../../types";

export const updatePeerLocationNodesOrders = async ({
  locationNodes,
  newDropOrder,
  workspaceId,
}: {
  locationNodes: ResourceNode[];
  newDropOrder: number;
  workspaceId: string;
}) => {
  const nodesToShift = locationNodes.filter((node) => node.order! >= newDropOrder);

  if (nodesToShift.length === 1) {
    await treeItemStateService.putOrder(nodesToShift[0].id, nodesToShift[0].order! + 1, workspaceId);
  }

  if (nodesToShift.length > 1) {
    await treeItemStateService.batchPutOrder(
      Object.fromEntries(nodesToShift.map((node) => [node.id, node.order! + 1])),
      workspaceId
    );
  }
};
