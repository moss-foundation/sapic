import {
  UseBatchCreateProjectResourceInput,
  UseBatchUpdateProjectResourceInput,
  UseDeleteProjectResourceInput,
} from "@/adapters";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { ListProjectResourcesOutput } from "@repo/ipc";
import { BatchCreateResourceOutput, BatchUpdateResourceOutput, DeleteResourceOutput } from "@repo/moss-project";
import { UseMutateAsyncFunction } from "@tanstack/react-query";
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
  batchCreateProjectResource: UseMutateAsyncFunction<
    BatchCreateResourceOutput,
    Error,
    UseBatchCreateProjectResourceInput,
    unknown
  >;
  batchUpdateProjectResource: UseMutateAsyncFunction<
    BatchUpdateResourceOutput,
    Error,
    UseBatchUpdateProjectResourceInput,
    unknown
  >;
  deleteProjectResource: UseMutateAsyncFunction<DeleteResourceOutput, Error, UseDeleteProjectResourceInput, unknown>;
  currentWorkspaceId: string;
  fetchResourcesForPath: (projectId: string, path: string) => Promise<ListProjectResourcesOutput>;
}

export const handleNodeOnFolderToAnotherProject =
  ({
    batchCreateProjectResource,
    batchUpdateProjectResource,
    deleteProjectResource,
    currentWorkspaceId,
    fetchResourcesForPath,
  }: HandleNodeOnFolderToAnotherProjectProps) =>
  async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode) => {
    const allResources = getAllNestedResources(sourceTreeNodeData.node);
    const resourcesPreparedForCreation = await prepareResourcesForCreation(allResources);

    const newOrder = locationTreeNodeData.node.childNodes.length + 1;

    await deleteProjectResource({
      projectId: sourceTreeNodeData.projectId,
      input: { id: sourceTreeNodeData.node.id },
    });

    await treeItemStateService.removeOrder(sourceTreeNodeData.node.id, currentWorkspaceId);
    await treeItemStateService.removeExpanded(sourceTreeNodeData.node.id, currentWorkspaceId);

    const updatedSourceResourcesPayload = siblingsAfterRemovalPayload({
      nodes: sourceTreeNodeData.parentNode.childNodes,
      removedNode: sourceTreeNodeData.node,
    });

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

    await batchCreateProjectResource({
      projectId: locationTreeNodeData.projectId,
      input: { resources: batchCreateResourceInput },
    });
    //TODO Create orders for created resources
    await fetchResourcesForPath(locationTreeNodeData.projectId, resolveParentPath(locationTreeNodeData.parentNode));
    await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));

    return;
  };
