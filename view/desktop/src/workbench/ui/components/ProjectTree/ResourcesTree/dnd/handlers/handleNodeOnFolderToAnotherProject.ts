import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { BatchCreateResourceOutput } from "@repo/moss-project";
import { join } from "@tauri-apps/api/path";

import { getAllNestedResources } from "../../getters/getAllNestedResources.ts";
import { ResourceNode } from "../../types.ts";
import { DragResourceNodeData } from "../types.dnd";
import { createResourceKind } from "../utils/createResourceKind.ts";
import { prepareResourcesForCreation, resolveParentPath } from "../utils/path";

interface HandleNodeOnFolderToAnotherProjectProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DragResourceNodeData;
  locationTreeNodeData: DragResourceNodeData;
}

export const handleNodeOnFolderToAnotherProject = async ({
  currentWorkspaceId,
  sourceTreeNodeData,
  locationTreeNodeData,
}: HandleNodeOnFolderToAnotherProjectProps) => {
  // 1) save source nodes
  const allFlatSourceResourceNodes = getAllNestedResources(sourceTreeNodeData.node);

  // 2) delete source nodes and states
  await deleteSourceNodesAndStates({
    sourceTreeNodeData,
    allFlatSourceResourceNodes,
    workspaceId: currentWorkspaceId,
  });

  // 3) update peer source nodes orders
  await updatePeerSourceNodesOrders({
    sourceNodes: sourceTreeNodeData.parentNode.childNodes,
    deletedNode: sourceTreeNodeData.node,
    workspaceId: currentWorkspaceId,
  });

  // 4) create location nodes
  const batchCreateResourceOutput = await createLocationNodes({
    allSourceResourceNodes: allFlatSourceResourceNodes,
    locationTreeNodeData,
  });

  // 5) assign source nodes states to location nodes states
  await assignSourceNodesStatesToLocationNodesStates({
    allSourceResourceNodes: allFlatSourceResourceNodes,
    batchCreateResourceOutput,
    workspaceId: currentWorkspaceId,
  });

  // 6) reload node paths
  await resourceService.list({
    projectId: locationTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(locationTreeNodeData.parentNode) },
  });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });
};

const deleteSourceNodesAndStates = async ({
  sourceTreeNodeData,
  allFlatSourceResourceNodes,
  workspaceId,
}: {
  sourceTreeNodeData: DragResourceNodeData;
  allFlatSourceResourceNodes: ResourceNode[];
  workspaceId: string;
}) => {
  await resourceService.delete(sourceTreeNodeData.projectId, {
    id: sourceTreeNodeData.node.id,
  });
  const deleteStatesFlatMap = allFlatSourceResourceNodes.flatMap((resource) => [
    treeItemStateService.removeOrder(resource.id, workspaceId),
    treeItemStateService.removeExpanded(resource.id, workspaceId),
  ]);
  await Promise.all(deleteStatesFlatMap);
};

const updatePeerSourceNodesOrders = async ({
  sourceNodes,
  deletedNode,
  workspaceId,
}: {
  sourceNodes: ResourceNode[];
  deletedNode: ResourceNode;
  workspaceId: string;
}) => {
  const updatedPeerNodes = sourceNodes
    .filter((node) => node.id !== deletedNode.id)
    .map((node, index) => ({
      ...node,
      order: index + 1,
    }));

  const nodesWithDifferentOrders = updatedPeerNodes.filter((node) => {
    const originalNode = sourceNodes.find((n) => n.id === node.id);
    return originalNode?.order !== node.order;
  });

  if (nodesWithDifferentOrders.length === 1) {
    await treeItemStateService.putOrder(nodesWithDifferentOrders[0].id, nodesWithDifferentOrders[0].order, workspaceId);
  }

  if (nodesWithDifferentOrders.length > 1) {
    await treeItemStateService.batchPutOrder(
      Object.fromEntries(nodesWithDifferentOrders.map((node) => [node.id, node.order])),
      workspaceId
    );
  }
};

const createLocationNodes = async ({
  allSourceResourceNodes,
  locationTreeNodeData,
}: {
  allSourceResourceNodes: ResourceNode[];
  locationTreeNodeData: DragResourceNodeData;
}) => {
  const newOrder = locationTreeNodeData.node.childNodes.length + 1;

  const sourceResourcesPreparedForCreation = await prepareResourcesForCreation(allSourceResourceNodes);
  const batchCreateResourceInput = await Promise.all(
    sourceResourcesPreparedForCreation.map(async (resource, index) => {
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

  return await resourceService.batchCreate(locationTreeNodeData.projectId, {
    resources: batchCreateResourceInput,
  });
};

const assignSourceNodesStatesToLocationNodesStates = async ({
  allSourceResourceNodes,
  batchCreateResourceOutput,
  workspaceId,
}: {
  allSourceResourceNodes: ResourceNode[];
  batchCreateResourceOutput: BatchCreateResourceOutput;
  workspaceId: string;
}) => {
  const orderMap = new Map<string, number>();
  const expandedMap = new Map<string, boolean>();

  batchCreateResourceOutput.resources.forEach((resource, index) => {
    const sourceResourceNode = allSourceResourceNodes[index];
    if (sourceResourceNode) {
      if (sourceResourceNode.order) {
        orderMap.set(resource.id, sourceResourceNode.order);
      }
      if (sourceResourceNode.expanded) {
        expandedMap.set(resource.id, sourceResourceNode.expanded);
      }
    }
  });

  if (orderMap.size > 0) {
    await treeItemStateService.batchPutOrder(Object.fromEntries(orderMap), workspaceId);
  }

  if (expandedMap.size > 0) {
    await treeItemStateService.batchPutExpanded(Object.fromEntries(expandedMap), workspaceId);
  }
};
