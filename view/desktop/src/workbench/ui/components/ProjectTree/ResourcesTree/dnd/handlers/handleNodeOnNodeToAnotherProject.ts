import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { BatchUpdateResourceEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";

import { DragNode, DropNode } from "../../../types";
import {
  createResourceKind,
  getAllNestedResources,
  prepareNestedDirResourcesForDrop,
  reorderedNodesForDifferentDirPayload,
  siblingsAfterRemovalPayload,
} from "../../../utils";

interface HandleNodeOnNodeToAnotherProjectProps {
  currentWorkspaceId: string;

  sourceTreeNodeData: DragNode;
  locationTreeNodeData: DropNode;
  operation: Operation;
}

export const handleNodeOnNodeToAnotherProject = async ({
  currentWorkspaceId,
  sourceTreeNodeData,
  locationTreeNodeData,
  operation,
}: HandleNodeOnNodeToAnotherProjectProps) => {
  const batchUpdateChannelEvent = new Channel<BatchUpdateResourceEvent>();

  const dropIndex =
    operation === "reorder-before" ? locationTreeNodeData.node.order! - 0.5 : locationTreeNodeData.node.order! + 0.5;

  const targetResourcesToUpdate = reorderedNodesForDifferentDirPayload({
    node: locationTreeNodeData.parentNode,
    newNode: sourceTreeNodeData.node,
    moveToIndex: dropIndex,
  }).filter((resource) => {
    if ("ITEM" in resource) {
      return resource.ITEM.id !== sourceTreeNodeData.node.id;
    } else {
      return resource.DIR.id !== sourceTreeNodeData.node.id;
    }
  });

  const updatedSourceResourcesPayload = siblingsAfterRemovalPayload({
    nodes: sourceTreeNodeData.parentNode.childNodes,
    removedNode: sourceTreeNodeData.node,
  });

  //Update items in source project
  await resourceService.batchUpdate(
    sourceTreeNodeData.projectId,
    {
      resources: updatedSourceResourcesPayload,
    },
    batchUpdateChannelEvent
  );

  for (const resource of updatedSourceResourcesPayload) {
    if ("ITEM" in resource) {
      await treeItemStateService.putExpanded(resource.ITEM.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
    } else if ("DIR" in resource) {
      await treeItemStateService.putOrder(resource.DIR.id, resource.DIR.order!, currentWorkspaceId);
      await treeItemStateService.putExpanded(resource.DIR.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
    }
  }

  //Update items in target project
  await resourceService.batchUpdate(
    locationTreeNodeData.projectId,
    {
      resources: targetResourcesToUpdate,
    },
    batchUpdateChannelEvent
  );

  for (const resource of targetResourcesToUpdate) {
    if ("ITEM" in resource) {
      await treeItemStateService.putExpanded(resource.ITEM.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
    } else if ("DIR" in resource) {
      await treeItemStateService.putOrder(resource.DIR.id, resource.DIR.order!, currentWorkspaceId);
      await treeItemStateService.putExpanded(resource.DIR.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
    }
  }
  //Delete item in source project
  await resourceService.delete(sourceTreeNodeData.projectId, {
    id: sourceTreeNodeData.node.id,
  });

  await treeItemStateService.removeOrder(sourceTreeNodeData.node.id, currentWorkspaceId);
  await treeItemStateService.removeExpanded(sourceTreeNodeData.node.id, currentWorkspaceId);

  const newDropOrder =
    operation === "reorder-before" ? locationTreeNodeData.node.order! : locationTreeNodeData.node.order! + 1;
  const allResources = getAllNestedResources(sourceTreeNodeData.node);
  const nestedResourcesWithoutName = await prepareNestedDirResourcesForDrop(allResources);
  const batchCreateResourceInput = await Promise.all(
    nestedResourcesWithoutName.map(async (resource, index) => {
      if (index === 0) {
        return createResourceKind({
          name: resource.name,
          path: "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : "",
          isAddingFolder: resource.kind === "Dir",
          order: newDropOrder,
          protocol: resource.protocol,
          class: "endpoint",
        });
      } else {
        const newResourcePath = await join(
          "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : "",
          resource.path.raw
        );
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

  const batchCreateResourceOutput = await resourceService.batchCreate(locationTreeNodeData.projectId, {
    resources: batchCreateResourceInput,
  });

  for (const resource of batchCreateResourceOutput.resources) {
    const resourceInput = batchCreateResourceInput.find((input) => {
      if ("ITEM" in input) {
        return input.ITEM.path === resource.path.raw && input.ITEM.name === resource.name;
      } else {
        return input.DIR.path === resource.path.raw && input.DIR.name === resource.name;
      }
    });

    if (resourceInput) {
      if ("ITEM" in resourceInput) {
        await treeItemStateService.putOrder(resource.id, resourceInput.ITEM.order, currentWorkspaceId);
        await treeItemStateService.putExpanded(resource.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
      } else {
        await treeItemStateService.putOrder(resource.id, resourceInput.DIR.order, currentWorkspaceId);
        await treeItemStateService.putExpanded(resource.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
      }
    }
  }

  await resourceService.list({
    projectId: locationTreeNodeData.projectId,
    mode: { "RELOAD_PATH": "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : "" },
  });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": "path" in sourceTreeNodeData.parentNode ? sourceTreeNodeData.parentNode.path.raw : "" },
  });
};
