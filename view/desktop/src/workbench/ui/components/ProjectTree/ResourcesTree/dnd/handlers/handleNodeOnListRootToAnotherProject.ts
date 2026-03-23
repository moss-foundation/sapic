import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { BatchUpdateResourceEvent } from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";

import { collectExpandedByResourceId } from "../../getters/collectExpandedByResourceId.ts";
import { getAllNestedResources } from "../../getters/getAllNestedResources.ts";
import { DragResourceNodeData, LocationResourcesListData } from "../types.dnd";
import {
  applyCrossProjectResourceRemapAfterBatchCreate,
  snapshotResourceDetailsForResourceIds,
} from "../utils/crossProjectResourceRecreate.ts";
import { createResourceKind } from "../utils/createResourceKind.ts";
import { prepareNestedDirResourcesForDrop, resolveParentPath, siblingsAfterRemovalPayload } from "../utils/path";

interface HandleNodeOnListRootToAnotherProjectProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DragResourceNodeData;
  locationResourcesListData: LocationResourcesListData;
}

export const handleNodeOnListRootToAnotherProject = async ({
  currentWorkspaceId,
  sourceTreeNodeData,
  locationResourcesListData,
}: HandleNodeOnListRootToAnotherProjectProps) => {
  const allResources = getAllNestedResources(sourceTreeNodeData.node);
  const resourcesWithoutName = await prepareNestedDirResourcesForDrop(allResources);

  const resourceDetailsSnapshot = snapshotResourceDetailsForResourceIds(allResources.map((r) => r.id));

  const newOrder = locationResourcesListData.data.rootResourcesNodes.length + 1;

  await resourceService.delete(sourceTreeNodeData.projectId, {
    id: sourceTreeNodeData.node.id,
  });

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

  const batchCreateResourceOutput = await resourceService.batchCreate(locationResourcesListData.data.projectId, {
    resources: batchCreateResourceInput,
  });

  applyCrossProjectResourceRemapAfterBatchCreate({
    allResources,
    batchCreateResourceInput,
    batchCreateOutput: batchCreateResourceOutput,
    destProjectId: locationResourcesListData.data.projectId,
    resourceDetailsSnapshot,
  });

  const expandedByOriginalId = collectExpandedByResourceId(sourceTreeNodeData.node);

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

    if (resourceInput) {
      const inputIndex = batchCreateResourceInput.indexOf(resourceInput);
      const originalResource = inputIndex >= 0 ? resourcesWithoutName[inputIndex] : undefined;
      expandedItems[resource.id] = originalResource ? (expandedByOriginalId.get(originalResource.id) ?? false) : false;
    } else {
      expandedItems[resource.id] = false;
    }
  }

  for (const resource of updatedSourceResourcesPayload) {
    if ("ITEM" in resource) {
      if ("order" in resource.ITEM) {
        orderItems[resource.ITEM.id] = resource.ITEM.order as number;
      }
    } else if ("DIR" in resource) {
      orderItems[resource.DIR.id] = resource.DIR.order!;
    }
  }

  await treeItemStateService.batchPutOrder(orderItems, currentWorkspaceId);
  await treeItemStateService.batchPutExpanded(expandedItems, currentWorkspaceId);

  await resourceService.list({ projectId: locationResourcesListData.data.projectId, mode: { "RELOAD_PATH": "" } });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });
};
