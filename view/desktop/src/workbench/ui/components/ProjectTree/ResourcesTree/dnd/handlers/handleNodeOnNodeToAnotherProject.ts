import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { BatchUpdateResourceEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";

import { collectExpandedByResourceId } from "../../getters/collectExpandedByResourceId.ts";
import { getAllNestedResources } from "../../getters/getAllNestedResources.ts";
import { DragResourceNodeData } from "../types.dnd";
import { createResourceKind } from "../utils/createResourceKind.ts";
import {
  prepareNestedDirResourcesForDrop,
  reorderedNodesForDifferentDirPayload,
  siblingsAfterRemovalPayload,
} from "../utils/path";

interface HandleNodeOnNodeToAnotherProjectProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DragResourceNodeData;
  locationTreeNodeData: DragResourceNodeData;
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

  const locationResourcesToUpdate = reorderedNodesForDifferentDirPayload({
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

  const sourceOrderItems: Record<string, number> = {};
  const sourceExpandedItems: Record<string, boolean> = {};

  const siblingExpandedById = new Map(sourceTreeNodeData.parentNode.childNodes.map((c) => [c.id, c.expanded] as const));

  for (const resource of updatedSourceResourcesPayload) {
    if ("ITEM" in resource) {
      sourceExpandedItems[resource.ITEM.id] = siblingExpandedById.get(resource.ITEM.id) ?? false;
      if ("order" in resource.ITEM) {
        sourceOrderItems[resource.ITEM.id] = resource.ITEM.order as number;
      }
    } else if ("DIR" in resource) {
      sourceOrderItems[resource.DIR.id] = resource.DIR.order!;
      sourceExpandedItems[resource.DIR.id] = siblingExpandedById.get(resource.DIR.id) ?? false;
    }
  }

  await Promise.all([
    Object.keys(sourceOrderItems).length > 0
      ? treeItemStateService.batchPutOrder(sourceOrderItems, currentWorkspaceId)
      : Promise.resolve(),
    Object.keys(sourceExpandedItems).length > 0
      ? treeItemStateService.batchPutExpanded(sourceExpandedItems, currentWorkspaceId)
      : Promise.resolve(),
  ]);

  //Update items in location project
  await resourceService.batchUpdate(
    locationTreeNodeData.projectId,
    {
      resources: locationResourcesToUpdate,
    },
    batchUpdateChannelEvent
  );

  const locationOrderItems: Record<string, number> = {};

  for (const resource of locationResourcesToUpdate) {
    if ("ITEM" in resource) {
      if ("order" in resource.ITEM) {
        locationOrderItems[resource.ITEM.id] = resource.ITEM.order as number;
      }
    } else if ("DIR" in resource) {
      locationOrderItems[resource.DIR.id] = resource.DIR.order!;
    }
  }

  await Promise.all([
    Object.keys(locationOrderItems).length > 0
      ? treeItemStateService.batchPutOrder(locationOrderItems, currentWorkspaceId)
      : Promise.resolve(),
  ]);

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

  const expandedByOriginalId = collectExpandedByResourceId(sourceTreeNodeData.node);

  const newOrderItems: Record<string, number> = {};
  const newExpandedItems: Record<string, boolean> = {};

  for (const created of batchCreateResourceOutput.resources) {
    const matchingInput = batchCreateResourceInput.find((input) => {
      const params = "ITEM" in input ? input.ITEM : input.DIR;
      return params.path === created.path.raw && params.name === created.name;
    });

    if (matchingInput) {
      const params = "ITEM" in matchingInput ? matchingInput.ITEM : matchingInput.DIR;
      if (params.order !== -1) {
        newOrderItems[created.id] = params.order;
      }
      const inputIndex = batchCreateResourceInput.indexOf(matchingInput);
      const originalResource = inputIndex >= 0 ? nestedResourcesWithoutName[inputIndex] : undefined;
      const expanded = originalResource ? (expandedByOriginalId.get(originalResource.id) ?? false) : false;
      newExpandedItems[created.id] = expanded;
    }
  }

  await Promise.all([
    Object.keys(newOrderItems).length > 0
      ? treeItemStateService.batchPutOrder(newOrderItems, currentWorkspaceId)
      : Promise.resolve(),
    Object.keys(newExpandedItems).length > 0
      ? treeItemStateService.batchPutExpanded(newExpandedItems, currentWorkspaceId)
      : Promise.resolve(),
  ]);

  await resourceService.list({
    projectId: locationTreeNodeData.projectId,
    mode: { "RELOAD_PATH": "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : "" },
  });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": "path" in sourceTreeNodeData.parentNode ? sourceTreeNodeData.parentNode.path.raw : "" },
  });
};
