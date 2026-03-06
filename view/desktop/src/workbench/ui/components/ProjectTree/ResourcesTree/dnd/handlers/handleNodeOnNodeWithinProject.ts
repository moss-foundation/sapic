import { UseBatchUpdateProjectResourceInput } from "@/adapters/tanstackQuery/resource/useBatchUpdateProjectResource";
import { computeSequentialOrders } from "@/utils/computeOrderUpdates";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { ListProjectResourcesOutput } from "@repo/ipc";
import { BatchUpdateResourceOutput } from "@repo/moss-project";
import { UseMutateAsyncFunction } from "@tanstack/react-query";

import { DragNode, DropNode } from "../../../types";
import { reorderedNodesForDifferentDirPayload, resolveParentPath, siblingsAfterRemovalPayload } from "../../../utils";

interface HandleNodeOnNodeWithinProjectProps {
  batchUpdateProjectResource: UseMutateAsyncFunction<
    BatchUpdateResourceOutput,
    Error,
    UseBatchUpdateProjectResourceInput,
    unknown
  >;
  currentWorkspaceId: string;
  fetchResourcesForPath: (projectId: string, path: string) => Promise<ListProjectResourcesOutput>;
  sourceTreeNodeData: DragNode;
  locationTreeNodeData: DropNode;
  operation: Operation;
}

export const handleNodeOnNodeWithinProject = async ({
  batchUpdateProjectResource,
  currentWorkspaceId,
  fetchResourcesForPath,
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

  await batchUpdateProjectResource({
    projectId: sourceTreeNodeData.projectId,
    resources: {
      resources: allResourcesToUpdate,
    },
  });

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

  await fetchResourcesForPath(locationTreeNodeData.projectId, resolveParentPath(locationTreeNodeData.parentNode));
  await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));
};
