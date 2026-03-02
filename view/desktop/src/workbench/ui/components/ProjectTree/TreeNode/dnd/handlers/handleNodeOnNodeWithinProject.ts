import { UseBatchUpdateProjectResourceInput } from "@/adapters/tanstackQuery/resource/useBatchUpdateProjectResource";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { BatchUpdateResourceOutput } from "@repo/moss-project";
import { UseMutateAsyncFunction } from "@tanstack/react-query";

import { ListProjectResourcesOutput } from "@repo/ipc";
import { DragNode, DropNode } from "../../../types";
import {
  reorderedNodesForDifferentDirPayload,
  reorderedNodesForSameDirPayload,
  resolveParentPath,
  siblingsAfterRemovalPayload,
} from "../../../utils";

interface HandleNodeOnNodeWithinProjectProps {
  batchUpdateProjectResource: UseMutateAsyncFunction<
    BatchUpdateResourceOutput,
    Error,
    UseBatchUpdateProjectResourceInput,
    unknown
  >;
  currentWorkspaceId: string;
  fetchResourcesForPath: (projectId: string, path: string) => Promise<ListProjectResourcesOutput>;
}

export const handleNodeOnNodeWithinProject =
  ({ batchUpdateProjectResource, currentWorkspaceId, fetchResourcesForPath }: HandleNodeOnNodeWithinProjectProps) =>
  async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode, operation: Operation) => {
    const dropIndex =
      operation === "reorder-before" ? locationTreeNodeData.node.order! - 0.5 : locationTreeNodeData.node.order! + 0.5;

    const inSameDir = sourceTreeNodeData.parentNode.id === locationTreeNodeData.parentNode.id;
    if (inSameDir) {
      const updatedSourceNodesPayload = reorderedNodesForSameDirPayload({
        nodes: sourceTreeNodeData.parentNode.childNodes,
        movedId: sourceTreeNodeData.node.id,
        moveToIndex: dropIndex,
      });

      await batchUpdateProjectResource({
        projectId: sourceTreeNodeData.projectId,
        resources: {
          resources: updatedSourceNodesPayload,
        },
      });

      const orderItems: Record<string, number> = {};
      const expandedItems: Record<string, boolean> = {};

      for (const resource of updatedSourceNodesPayload) {
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

      await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));

      return;
    }

    const targetResourcesToUpdate = reorderedNodesForDifferentDirPayload({
      node: locationTreeNodeData.parentNode,
      newNode: sourceTreeNodeData.node,
      moveToIndex: dropIndex,
    });

    const sourceResourcesToUpdate = siblingsAfterRemovalPayload({
      nodes: sourceTreeNodeData.parentNode.childNodes,
      removedNode: sourceTreeNodeData.node,
    });

    const allResourcesToUpdate = [...targetResourcesToUpdate, ...sourceResourcesToUpdate];

    await batchUpdateProjectResource({
      projectId: sourceTreeNodeData.projectId,
      resources: {
        resources: allResourcesToUpdate,
      },
    });

    const orderItems: Record<string, number> = {};
    const expandedItems: Record<string, boolean> = {};

    for (const resource of allResourcesToUpdate) {
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

    await fetchResourcesForPath(locationTreeNodeData.projectId, resolveParentPath(locationTreeNodeData.parentNode));
    await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));
  };
