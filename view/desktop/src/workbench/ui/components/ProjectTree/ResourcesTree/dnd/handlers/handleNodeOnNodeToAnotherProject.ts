import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { BatchCreateResourceOutput } from "@repo/moss-project";
import { join } from "@tauri-apps/api/path";

import { getAllNestedResources } from "../../getters/getAllNestedResources.ts";
import { ResourceNode } from "../../types.ts";
import { DragResourceNodeData, ResourceNodeWithDetails } from "../types.dnd";
import { createResourceKind } from "../utils/createResourceKind.ts";
import { prepareResourcesForCreation, resolveParentPath } from "../utils/path";
import { remapOldIdsForDockviewLayout } from "../utils/remapResourceIdsInSerializedDockview.ts";

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
  const newDropOrder =
    operation === "reorder-before" ? locationTreeNodeData.node.order! : locationTreeNodeData.node.order! + 1;

  // 1) save source nodes
  const allFlatSourceResourceNodes = await getAllNestedResources({
    node: sourceTreeNodeData.node,
    projectId: sourceTreeNodeData.projectId,
  });

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

  // 4) update peer location nodes orders
  await updatePeerLocationNodesOrders({
    locationNodes: locationTreeNodeData.parentNode.childNodes,
    newDropOrder,
    workspaceId: currentWorkspaceId,
  });

  // 5) create location nodes
  const batchCreateResourceOutput = await createLocationNodes({
    allSourceResourceNodes: allFlatSourceResourceNodes,
    locationTreeNodeData,
    newDropOrder,
  });

  // 6) assign source nodes states to location nodes states
  await assignSourceNodesStatesToLocationNodesStates({
    allSourceResourceNodes: allFlatSourceResourceNodes,
    batchCreateResourceOutput,
    workspaceId: currentWorkspaceId,
    newDropOrder,
  });

  // 7) update resourceDetailsCollection
  updateResourceDetailsCollection({
    allFlatSourceResourceNodes,
    batchCreateResourceOutput,
  });

  // 8) remap resource ids in dockview
  remapOldIdsForDockviewLayout({
    allFlatSourceResourceNodes,
    batchCreateResourceOutput,
    destProjectId: locationTreeNodeData.projectId,
  });

  // 9) reload node paths
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
  allFlatSourceResourceNodes: ResourceNodeWithDetails[];
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

const updatePeerLocationNodesOrders = async ({
  locationNodes,
  newDropOrder,
  workspaceId,
}: {
  locationNodes: ResourceNode[];
  newDropOrder: number;
  workspaceId: string;
}) => {
  const nodesToShift = locationNodes.filter((node) => node.order! >= newDropOrder);

  if (nodesToShift.length === 1) {
    await treeItemStateService.putOrder(nodesToShift[0].id, nodesToShift[0].order! + 1, workspaceId);
  }

  if (nodesToShift.length > 1) {
    await treeItemStateService.batchPutOrder(
      Object.fromEntries(nodesToShift.map((node) => [node.id, node.order! + 1])),
      workspaceId
    );
  }
};

const createLocationNodes = async ({
  allSourceResourceNodes,
  locationTreeNodeData,
  newDropOrder,
}: {
  allSourceResourceNodes: ResourceNodeWithDetails[];
  locationTreeNodeData: DragResourceNodeData;
  newDropOrder: number;
}) => {
  const parentPath = resolveParentPath(locationTreeNodeData.parentNode);
  const sourceResourcesPreparedForCreation = await prepareResourcesForCreation(allSourceResourceNodes);

  const batchCreateResourceInput = await Promise.all(
    sourceResourcesPreparedForCreation.map(async (resource, index) => {
      if (index === 0) {
        return createResourceKind({
          name: resource.name,
          path: parentPath,
          isAddingFolder: resource.kind === "Dir",
          order: newDropOrder,
          protocol: resource.protocol,
          headers: resource.details.headers,
          queryParams: resource.details.queryParams,
          pathParams: resource.details.pathParams,
          body: resource.details.body,
          class: "endpoint",
        });
      } else {
        const newResourcePath = await join(parentPath, resource.path.raw);
        return createResourceKind({
          name: resource.name,
          path: newResourcePath,
          isAddingFolder: resource.kind === "Dir",
          order: -1,
          protocol: resource.protocol,
          headers: resource.details.headers,
          queryParams: resource.details.queryParams,
          pathParams: resource.details.pathParams,
          body: resource.details.body,
          class: "endpoint",
        });
      }
    })
  );

  return await resourceService.batchCreate(locationTreeNodeData.projectId, {
    resources: batchCreateResourceInput,
  });
};

const updateResourceDetailsCollection = ({
  allFlatSourceResourceNodes,
  batchCreateResourceOutput,
}: {
  allFlatSourceResourceNodes: ResourceNodeWithDetails[];
  batchCreateResourceOutput: BatchCreateResourceOutput;
}) => {
  batchCreateResourceOutput.resources.forEach((newResource, index) => {
    const sourceResource = allFlatSourceResourceNodes[index];
    if (sourceResource?.collectionDetails) {
      resourceDetailsCollection.insert({ ...sourceResource.collectionDetails, id: newResource.id });
      resourceDetailsCollection.delete(sourceResource.id);
    }
  });
};

const assignSourceNodesStatesToLocationNodesStates = async ({
  allSourceResourceNodes,
  batchCreateResourceOutput,
  workspaceId,
  newDropOrder,
}: {
  allSourceResourceNodes: ResourceNodeWithDetails[];
  batchCreateResourceOutput: BatchCreateResourceOutput;
  workspaceId: string;
  newDropOrder: number;
}) => {
  const orderMap = new Map<string, number>();
  const expandedMap = new Map<string, boolean>();

  batchCreateResourceOutput.resources.forEach((resource, index) => {
    const sourceResourceNode = allSourceResourceNodes[index];
    if (sourceResourceNode) {
      if (index === 0) {
        orderMap.set(resource.id, newDropOrder);
      } else {
        if (sourceResourceNode.order) orderMap.set(resource.id, sourceResourceNode.order);
        if (sourceResourceNode.expanded) expandedMap.set(resource.id, sourceResourceNode.expanded);
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
