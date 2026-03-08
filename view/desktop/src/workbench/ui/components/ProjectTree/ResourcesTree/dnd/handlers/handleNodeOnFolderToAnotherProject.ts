import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { BatchUpdateResourceEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";

import { DragNode, DropNode } from "../../../types";
import {
  createResourceKind,
  getAllNestedResources,
  prepareResourcesForCreation,
  resolveParentPath,
  siblingsAfterRemovalPayload,
} from "../../../utils";

interface HandleNodeOnFolderToAnotherProjectProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DragNode;
  locationTreeNodeData: DropNode;
}

export const handleNodeOnFolderToAnotherProject = async ({
  currentWorkspaceId,
  sourceTreeNodeData,
  locationTreeNodeData,
}: HandleNodeOnFolderToAnotherProjectProps) => {
  const allResources = getAllNestedResources(sourceTreeNodeData.node);
  const resourcesPreparedForCreation = await prepareResourcesForCreation(allResources);

  const newOrder = locationTreeNodeData.node.childNodes.length + 1;

  await resourceService.delete(sourceTreeNodeData.projectId, {
    id: sourceTreeNodeData.node.id,
  });

  await treeItemStateService.removeOrder(sourceTreeNodeData.node.id, currentWorkspaceId);
  await treeItemStateService.removeExpanded(sourceTreeNodeData.node.id, currentWorkspaceId);

  const updatedSourceResourcesPayload = siblingsAfterRemovalPayload({
    nodes: sourceTreeNodeData.parentNode.childNodes,
    removedNode: sourceTreeNodeData.node,
  });

  const channelEvent = new Channel<BatchUpdateResourceEvent>();
  await resourceService.batchUpdate(
    sourceTreeNodeData.projectId,
    {
      resources: updatedSourceResourcesPayload,
    },
    channelEvent
  );

  for (const resource of updatedSourceResourcesPayload) {
    if ("ITEM" in resource) {
      await treeItemStateService.putExpanded(resource.ITEM.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
    } else if ("DIR" in resource) {
      await treeItemStateService.putOrder(resource.DIR.id, resource.DIR.order!, currentWorkspaceId);
      await treeItemStateService.putExpanded(resource.DIR.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
    }
  }

  const batchCreateResourceInput = await Promise.all(
    resourcesPreparedForCreation.map(async (resource, index) => {
      if (index === 0) {
        return createResourceKind({
          name: resource.name,
          path: locationTreeNodeData.node.path.raw,
          isAddingFolder: resource.kind === "Dir",
          order: newOrder,
          protocol: resource.protocol,
          class: "endpoint",
        });
      } else {
        const newResourcePath = await join(locationTreeNodeData.node.path.raw, resource.path.raw);
        return createResourceKind({
          name: resource.name,
          path: newResourcePath,
          isAddingFolder: resource.kind === "Dir",
          order: -1,
          protocol: resource.protocol,
          class: "endpoint",
        });
      }
    })
  );

  await resourceService.batchCreate(locationTreeNodeData.projectId, {
    resources: batchCreateResourceInput,
  });
  //TODO Create orders for created resources
  await resourceService.list({
    projectId: locationTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(locationTreeNodeData.parentNode) },
  });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });

  return;
};
