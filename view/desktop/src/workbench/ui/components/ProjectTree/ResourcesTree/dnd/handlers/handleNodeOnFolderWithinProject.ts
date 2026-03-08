import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { BatchUpdateResourceEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";

import { DragNode, DropNode } from "../../../types";
import {
  makeDirUpdatePayload,
  makeItemUpdatePayload,
  resolveParentPath,
  siblingsAfterRemovalPayload,
} from "../../../utils";

interface HandleNodeOnFolderWithinProjectProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DragNode;
  locationTreeNodeData: DropNode;
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

  await treeItemStateService.putOrder(sourceTreeNodeData.node.id, newOrder, currentWorkspaceId);
  await treeItemStateService.putExpanded(
    sourceTreeNodeData.node.id,
    sourceTreeNodeData.node.expanded,
    currentWorkspaceId
  );

  await resourceService.list({
    projectId: locationTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(locationTreeNodeData.parentNode) },
  });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });
};
