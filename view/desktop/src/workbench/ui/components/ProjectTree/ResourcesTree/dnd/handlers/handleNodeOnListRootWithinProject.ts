import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { BatchUpdateResourceEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";

import {
  makeDirUpdatePayload,
  makeItemUpdatePayload,
  resolveParentPath,
  siblingsAfterRemovalPayload,
} from "../../../utils";
import { DragResourceNodeData, LocationResourcesListData } from "../types.dnd";

interface HandleNodeOnListRootWithinProjectProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DragResourceNodeData;
  locationResourcesListData: LocationResourcesListData;
}

export const handleNodeOnListRootWithinProject = async ({
  currentWorkspaceId,
  sourceTreeNodeData,
  locationResourcesListData,
}: HandleNodeOnListRootWithinProjectProps) => {
  const newOrder = locationResourcesListData.data.rootResourcesNodes.length + 1;

  const sourceNodeUpdate =
    sourceTreeNodeData.node.kind === "Dir"
      ? makeDirUpdatePayload({
          id: sourceTreeNodeData.node.id,
          path: "",
          order: newOrder,
        })
      : makeItemUpdatePayload({
          id: sourceTreeNodeData.node.id,
          path: "",
          order: newOrder,
        });

  const nodesToUpdate = siblingsAfterRemovalPayload({
    nodes: sourceTreeNodeData.parentNode.childNodes,
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
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": "" },
  });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });
};
