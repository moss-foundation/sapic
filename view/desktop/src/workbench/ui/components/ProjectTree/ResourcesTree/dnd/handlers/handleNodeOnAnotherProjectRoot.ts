import {
  UseBatchCreateProjectResourceInput,
  UseBatchUpdateProjectResourceInput,
  UseDeleteProjectResourceInput,
} from "@/adapters";
import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { ListProjectResourcesOutput } from "@repo/ipc";
import { BatchCreateResourceOutput, BatchUpdateResourceOutput, DeleteResourceOutput } from "@repo/moss-project";
import { UseMutateAsyncFunction } from "@tanstack/react-query";
import { join } from "@tauri-apps/api/path";

import { DragNode, DropRootNode } from "../../../types";
import {
  createResourceKind,
  getAllNestedResources,
  prepareNestedDirResourcesForDrop,
  resolveParentPath,
  siblingsAfterRemovalPayload,
} from "../../../utils";

interface HandleNodeOnAnotherProjectRootProps {
  deleteProjectResource: UseMutateAsyncFunction<DeleteResourceOutput, Error, UseDeleteProjectResourceInput, unknown>;
  batchUpdateProjectResource: UseMutateAsyncFunction<
    BatchUpdateResourceOutput,
    Error,
    UseBatchUpdateProjectResourceInput,
    unknown
  >;
  batchCreateProjectResource: UseMutateAsyncFunction<
    BatchCreateResourceOutput,
    Error,
    UseBatchCreateProjectResourceInput,
    unknown
  >;
  currentWorkspaceId: string;
  fetchResourcesForPath: (projectId: string, path: string) => Promise<ListProjectResourcesOutput>;
  sourceTreeNodeData: DragNode;
  locationTreeRootNodeData: DropRootNode;
}

export const handleNodeOnAnotherProjectRoot = async ({
  deleteProjectResource,
  batchUpdateProjectResource,
  batchCreateProjectResource,
  currentWorkspaceId,
  fetchResourcesForPath,
  sourceTreeNodeData,
  locationTreeRootNodeData,
}: HandleNodeOnAnotherProjectRootProps) => {
  const allResources = getAllNestedResources(sourceTreeNodeData.node);
  const resourcesWithoutName = await prepareNestedDirResourcesForDrop(allResources);

  const newOrder = locationTreeRootNodeData.node.childNodes.length + 1;

  await deleteProjectResource({
    projectId: sourceTreeNodeData.projectId,
    input: { id: sourceTreeNodeData.node.id },
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

  await batchUpdateProjectResource({
    projectId: sourceTreeNodeData.projectId,
    resources: {
      resources: updatedSourceResourcesPayload,
    },
  });

  const batchCreateResourceOutput = await batchCreateProjectResource({
    projectId: locationTreeRootNodeData.projectId,
    input: {
      resources: batchCreateResourceInput,
    },
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

  await fetchResourcesForPath(locationTreeRootNodeData.projectId, "");
  await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));
};
