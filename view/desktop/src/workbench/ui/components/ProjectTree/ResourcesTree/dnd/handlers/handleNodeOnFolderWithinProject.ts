import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { BatchUpdateResourceEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";

import { DraggedResourceNode } from "../../../types";
import {
  makeDirUpdatePayload,
  makeItemUpdatePayload,
  resolveParentPath,
  siblingsAfterRemovalPayload,
} from "../../../utils";

interface HandleNodeOnFolderWithinProjectProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DraggedResourceNode;
  locationTreeNodeData: DraggedResourceNode;
}

export const handleNodeOnFolderWithinProject = async ({
  currentWorkspaceId,
  sourceTreeNodeData,
  locationTreeNodeData,
}: HandleNodeOnFolderWithinProjectProps) => {
  const newOrder = locationTreeNodeData.node.childNodes.length + 1;

  const sourceNodeUpdate =
    sourceTreeNodeData.node.kind === "Dir"
      ? makeDirUpdatePayload({
          id: sourceTreeNodeData.node.id,
          path: locationTreeNodeData.node.path.raw,
          order: newOrder,
        })
      : makeItemUpdatePayload({
          id: sourceTreeNodeData.node.id,
          path: locationTreeNodeData.node.path.raw,
          order: newOrder,
        });

  const sourceParentNodes = sourceTreeNodeData.parentNode.childNodes;
  const nodesToUpdate = siblingsAfterRemovalPayload({
    nodes: sourceParentNodes,
    removedNode: sourceTreeNodeData.node,
  });

  const allUpdates = [sourceNodeUpdate, ...nodesToUpdate];

  const channelEvent = new Channel<BatchUpdateResourceEvent>();
  await resourceService.batchUpdate(
    sourceTreeNodeData.projectId,
    {
      resources: allUpdates,
    },
    channelEvent
  );

  const orderItems: Record<string, number> = {};
  const expandedItems: Record<string, boolean> = {};

  for (const resource of allUpdates) {
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
