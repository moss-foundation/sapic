import { resourceService } from "@/domains/resource/resourceService";
import { computeSequentialOrders } from "@/utils/computeOrderUpdates";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { UpdateResourceInput } from "@repo/moss-project";

import { ResourceNode } from "../../types.ts";
import { resolveParentPath } from "../handlerOperations/path.ts";
import { updatePeerLocationNodesOrders } from "../handlerOperations/updatePeerLocationNodesOrders.ts";
import { updatePeerSourceNodesOrders } from "../handlerOperations/updatePeerSourceNodesOrders.ts";
import { DragResourceNodeData } from "../types.dnd";

interface HandleNodeOnNodeWithinProjectProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DragResourceNodeData;
  locationTreeNodeData: DragResourceNodeData;
  operation: Operation;
}

export const handleNodeOnNodeWithinProject = async ({
  currentWorkspaceId,
  sourceTreeNodeData,
  locationTreeNodeData,
  operation,
}: HandleNodeOnNodeWithinProjectProps) => {
  const inSameDir = sourceTreeNodeData.parentNode.id === locationTreeNodeData.parentNode.id;
  if (inSameDir) {
    await reorderNodesWithinDir({
      sourceNode: sourceTreeNodeData.node,
      locationNode: locationTreeNodeData.node,
      allNodes: sourceTreeNodeData.parentNode.childNodes,
      operation,
      workspaceId: currentWorkspaceId,
    });
  } else {
    const dropOrder =
      operation === "reorder-before" ? locationTreeNodeData.node.order! : locationTreeNodeData.node.order! + 1;

    // 1) update source node path
    await updateSourceNodePath({
      sourceTreeNodeData,
      locationTreeNodeData,
    });

    // 2) update peer source nodes orders
    await updatePeerSourceNodesOrders({
      sourceNodes: sourceTreeNodeData.parentNode.childNodes,
      deletedNode: sourceTreeNodeData.node,
      workspaceId: currentWorkspaceId,
    });

    // 3) update peer location nodes orders
    await updatePeerLocationNodesOrders({
      locationNodes: locationTreeNodeData.parentNode.childNodes,
      newDropOrder: dropOrder,
      workspaceId: currentWorkspaceId,
    });

    // 4) update root source node order
    await treeItemStateService.putOrder(sourceTreeNodeData.node.id, dropOrder, currentWorkspaceId);

    // 5) reload node paths
    await resourceService.list({
      projectId: locationTreeNodeData.projectId,
      mode: { "RELOAD_PATH": resolveParentPath(locationTreeNodeData.parentNode) },
    });
    await resourceService.list({
      projectId: sourceTreeNodeData.projectId,
      mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
    });
  }
};

const reorderNodesWithinDir = async ({
  sourceNode,
  locationNode,
  allNodes,
  operation,
  workspaceId,
}: {
  sourceNode: ResourceNode;
  locationNode: ResourceNode;
  allNodes: ResourceNode[];
  operation: Operation;
  workspaceId: string;
}) => {
  const sortedNodes = sortObjectsByOrder(allNodes);
  const locationIndex = sortedNodes.findIndex((n) => n.id === locationNode.id);
  const dropOrder = operation === "reorder-before" ? locationIndex : locationIndex + 1;

  const reorderedNodes = [
    ...sortedNodes.slice(0, dropOrder).filter((n) => n.id !== sourceNode.id),
    sourceNode,
    ...sortedNodes.slice(dropOrder).filter((n) => n.id !== sourceNode.id),
  ];

  const updates = computeSequentialOrders(reorderedNodes);
  if (Object.keys(updates).length === 0) return;

  await treeItemStateService.batchPutOrder(updates, workspaceId);
};

const updateSourceNodePath = async ({
  sourceTreeNodeData,
  locationTreeNodeData,
}: {
  sourceTreeNodeData: DragResourceNodeData;
  locationTreeNodeData: DragResourceNodeData;
}) => {
  const newPath = resolveParentPath(locationTreeNodeData.parentNode);
  const updatePayload: UpdateResourceInput =
    sourceTreeNodeData.node.kind === "Dir"
      ? {
          DIR: {
            id: sourceTreeNodeData.node.id,
            path: newPath,
          },
        }
      : {
          ITEM: {
            id: sourceTreeNodeData.node.id,
            path: newPath,
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: [],
            pathParamsToRemove: [],
            queryParamsToAdd: [],
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
          },
        };

  await resourceService.update(sourceTreeNodeData.projectId, updatePayload);
};
