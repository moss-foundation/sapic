import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { BatchUpdateResourceEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";

import { DraggedResourceNode, DropRootNode } from "../../../types";
import {
  createResourceKind,
  getAllNestedResources,
  prepareNestedDirResourcesForDrop,
  resolveParentPath,
  siblingsAfterRemovalPayload,
} from "../../../utils";

interface HandleNodeOnAnotherProjectRootProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DraggedResourceNode;
  locationTreeRootNodeData: DropRootNode;
}

export const handleNodeOnAnotherProjectRoot = async ({
  currentWorkspaceId,
  sourceTreeNodeData,
  locationTreeRootNodeData,
}: HandleNodeOnAnotherProjectRootProps) => {
  const allResources = getAllNestedResources(sourceTreeNodeData.node);
  const resourcesWithoutName = await prepareNestedDirResourcesForDrop(allResources);

  const newOrder = locationTreeRootNodeData.node.resourcesTree.childNodes.length + 1;

  await resourceService.delete(sourceTreeNodeData.projectId, {
    id: sourceTreeNodeData.node.id,
  });

  for (const resource of allResources) {
    if (resourceSummariesCollection.has(resource.id)) {
      resourceSummariesCollection.delete(resource.id);
    }
    if (resourceDetailsCollection.has(resource.id)) {
      resourceDetailsCollection.delete(resource.id);
    }
  }

  const updatedSourceResourcesPayload = siblingsAfterRemovalPayload({
    nodes: sourceTreeNodeData.parentNode.childNodes,
    removedNode: sourceTreeNodeData.node,
  });

  const batchCreateResourceInput = await Promise.all(
    resourcesWithoutName.map(async (resource, index) => {
      const newResourcePath = await join(resource.path.raw);

      if (index === 0) {
        return createResourceKind({
          name: resource.name,
          path: "",
          class: "endpoint",
          isAddingFolder: resource.kind === "Dir",
          order: newOrder,
          protocol: resource.protocol,
        });
      } else {
        return createResourceKind({
          name: resource.name,
          path: newResourcePath,
          class: "endpoint",
          isAddingFolder: resource.kind === "Dir",
          order: -1,
          protocol: resource.protocol,
        });
      }
    })
  );

  const channelEvent = new Channel<BatchUpdateResourceEvent>();
  await resourceService.batchUpdate(
    sourceTreeNodeData.projectId,
    {
      resources: updatedSourceResourcesPayload,
    },
    channelEvent
  );

  const batchCreateResourceOutput = await resourceService.batchCreate(locationTreeRootNodeData.projectId, {
    resources: batchCreateResourceInput,
  });

  const orderItems: Record<string, number> = {};
  const expandedItems: Record<string, boolean> = {};

  for (const resource of batchCreateResourceOutput.resources) {
    const resourceInput = batchCreateResourceInput.find((input) => {
      if ("ITEM" in input) {
        return input.ITEM.path === resource.path.raw && input.ITEM.name === resource.name;
      } else {
        return input.DIR.path === resource.path.raw && input.DIR.name === resource.name;
      }
    });

    orderItems[resource.id] = resourceInput
      ? "ITEM" in resourceInput
        ? resourceInput.ITEM.order
        : resourceInput.DIR.order
      : -1;
    expandedItems[resource.id] = sourceTreeNodeData.node.expanded;
  }

  await treeItemStateService.batchPutOrder(orderItems, currentWorkspaceId);
  await treeItemStateService.batchPutExpanded(expandedItems, currentWorkspaceId);

  await resourceService.list({ projectId: locationTreeRootNodeData.projectId, mode: { "RELOAD_PATH": "" } });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });
};
