import {
  UseBatchCreateProjectResourceInput,
  UseBatchUpdateProjectResourceInput,
  UseDeleteProjectResourceInput,
} from "@/adapters";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { ListProjectResourcesOutput } from "@repo/ipc";
import { BatchCreateResourceOutput, BatchUpdateResourceOutput, DeleteResourceOutput } from "@repo/moss-project";
import { UseMutateAsyncFunction } from "@tanstack/react-query";
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
  batchUpdateProjectResource: UseMutateAsyncFunction<
    BatchUpdateResourceOutput,
    Error,
    UseBatchUpdateProjectResourceInput,
    unknown
  >;
  deleteProjectResource: UseMutateAsyncFunction<DeleteResourceOutput, Error, UseDeleteProjectResourceInput, unknown>;
  currentWorkspaceId: string;
  fetchResourcesForPath: (projectId: string, path: string) => Promise<ListProjectResourcesOutput>;
  batchCreateProjectResource: UseMutateAsyncFunction<
    BatchCreateResourceOutput,
    Error,
    UseBatchCreateProjectResourceInput,
    unknown
  >;
  sourceTreeNodeData: DragNode;
  locationTreeNodeData: DropNode;
  operation: Operation;
}

export const handleNodeOnNodeToAnotherProject = async ({
  batchUpdateProjectResource,
  deleteProjectResource,
  currentWorkspaceId,
  fetchResourcesForPath,
  batchCreateProjectResource,
  sourceTreeNodeData,
  locationTreeNodeData,
  operation,
}: HandleNodeOnNodeToAnotherProjectProps) => {
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
  await batchUpdateProjectResource({
    projectId: sourceTreeNodeData.projectId,
    resources: {
      resources: updatedSourceResourcesPayload,
    },
  });

  for (const resource of updatedSourceResourcesPayload) {
    if ("ITEM" in resource) {
      await treeItemStateService.putExpanded(resource.ITEM.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
    } else if ("DIR" in resource) {
      await treeItemStateService.putOrder(resource.DIR.id, resource.DIR.order!, currentWorkspaceId);
      await treeItemStateService.putExpanded(resource.DIR.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
    }
  }

  //Update items in target project
  await batchUpdateProjectResource({
    projectId: locationTreeNodeData.projectId,
    resources: {
      resources: targetResourcesToUpdate,
    },
  });

  for (const resource of targetResourcesToUpdate) {
    if ("ITEM" in resource) {
      await treeItemStateService.putExpanded(resource.ITEM.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
    } else if ("DIR" in resource) {
      await treeItemStateService.putOrder(resource.DIR.id, resource.DIR.order!, currentWorkspaceId);
      await treeItemStateService.putExpanded(resource.DIR.id, sourceTreeNodeData.node.expanded, currentWorkspaceId);
    }
  }
  //Delete item in source project
  await deleteProjectResource({
    projectId: sourceTreeNodeData.projectId,
    input: { id: sourceTreeNodeData.node.id },
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

  const batchCreateResourceOutput = await batchCreateProjectResource({
    projectId: locationTreeNodeData.projectId,
    input: {
      resources: batchCreateResourceInput,
    },
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

  await fetchResourcesForPath(
    locationTreeNodeData.projectId,
    "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : ""
  );
  await fetchResourcesForPath(
    sourceTreeNodeData.projectId,
    "path" in sourceTreeNodeData.parentNode ? sourceTreeNodeData.parentNode.path.raw : ""
  );
};
