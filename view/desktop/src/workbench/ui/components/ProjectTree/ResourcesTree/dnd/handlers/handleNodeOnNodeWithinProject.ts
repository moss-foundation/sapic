import { resourceService } from "@/domains/resource/resourceService";
import { computeSequentialOrders } from "@/utils/computeOrderUpdates";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { BatchUpdateResourceEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";

import { reorderedNodesForDifferentDirPayload, resolveParentPath, siblingsAfterRemovalPayload } from "../../../utils";
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
  const dropIndex =
    operation === "reorder-before" ? locationTreeNodeData.node.order! - 0.5 : locationTreeNodeData.node.order! + 0.5;

  const inSameDir = sourceTreeNodeData.parentNode.id === locationTreeNodeData.parentNode.id;
  if (inSameDir) {
    const sortedNodes = sortObjectsByOrder(sourceTreeNodeData.parentNode.childNodes);
    const targetIndex = sortedNodes.findIndex((n) => n.id === locationTreeNodeData.node.id);
    const dropOrder = operation === "reorder-before" ? targetIndex : targetIndex + 1;

    const reorderedNodes = [
      ...sortedNodes.slice(0, dropOrder).filter((n) => n.id !== sourceTreeNodeData.node.id),
      sourceTreeNodeData.node,
      ...sortedNodes.slice(dropOrder).filter((n) => n.id !== sourceTreeNodeData.node.id),
    ];

    const updates = computeSequentialOrders(reorderedNodes);
    if (Object.keys(updates).length === 0) return;

    await treeItemStateService.batchPutOrder(updates, currentWorkspaceId);
    return;
  }

  const targetResourcesToUpdate = reorderedNodesForDifferentDirPayload({
    node: locationTreeNodeData.parentNode,
    newNode: sourceTreeNodeData.node,
    moveToIndex: dropIndex,
  });

  const sourceResourcesToUpdate = siblingsAfterRemovalPayload({
    nodes: sourceTreeNodeData.parentNode.childNodes,
    removedNode: sourceTreeNodeData.node,
  });

  const allResourcesToUpdate = [...targetResourcesToUpdate, ...sourceResourcesToUpdate];

  const channelEvent = new Channel<BatchUpdateResourceEvent>();
  await resourceService.batchUpdate(
    sourceTreeNodeData.projectId,
    {
      resources: allResourcesToUpdate,
    },
    channelEvent
  );

  const orderItems: Record<string, number> = {};
  const expandedItems: Record<string, boolean> = {};

  for (const resource of allResourcesToUpdate) {
    if ("ITEM" in resource) {
      expandedItems[resource.ITEM.id] = sourceTreeNodeData.node.expanded;
      if ("order" in resource.ITEM) {
        orderItems[resource.ITEM.id] = resource.ITEM.order as number;
      }
    } else if ("DIR" in resource) {
      orderItems[resource.DIR.id] = resource.DIR.order!;
      expandedItems[resource.DIR.id] = sourceTreeNodeData.node.expanded;
    }
  }

  await Promise.all([
    Object.keys(orderItems).length > 0
      ? treeItemStateService.batchPutOrder(orderItems, currentWorkspaceId)
      : Promise.resolve(),
    Object.keys(expandedItems).length > 0
      ? treeItemStateService.batchPutExpanded(expandedItems, currentWorkspaceId)
      : Promise.resolve(),
  ]);

  await resourceService.list({
    projectId: locationTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(locationTreeNodeData.parentNode) },
  });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });
};
