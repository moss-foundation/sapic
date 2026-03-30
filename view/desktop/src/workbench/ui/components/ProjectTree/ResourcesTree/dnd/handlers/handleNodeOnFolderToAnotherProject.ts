import { resourceService } from "@/domains/resource/resourceService";
import { join } from "@tauri-apps/api/path";

import { getAllNestedResources } from "../../getters/getAllNestedResources.ts";
import { assignSourceNodesStatesToLocationNodesStates } from "../handlerOperations/assignSourceNodesStatesToLocationNodesStates.ts";
import { createResourceKind } from "../handlerOperations/createResourceKind.ts";
import { deleteSourceNodesAndStates } from "../handlerOperations/deleteSourceNodesAndStates.ts";
import { prepareResourcesForCreation, resolveParentPath } from "../handlerOperations/path.ts";
import { remapOldIdsForDockviewLayout } from "../handlerOperations/remapResourceIdsInSerializedDockview.ts";
import { updatePeerSourceNodesOrders } from "../handlerOperations/updatePeerSourceNodesOrders.ts";
import { updateResourceDetailsCollection } from "../handlerOperations/updateResourceDetailsCollection.ts";
import { DragResourceNodeData, ResourceNodeWithDetails } from "../types.dnd";

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
  const newRootSourceNodeOrder = locationTreeNodeData.node.childNodes.length + 1;
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

  // 4) create location nodes
  const batchCreateResourceOutput = await createLocationNodes({
    allSourceResourceNodes: allFlatSourceResourceNodes,
    locationTreeNodeData,
    newRootSourceNodeOrder,
  });

  // 5) assign source nodes states to location nodes states
  await assignSourceNodesStatesToLocationNodesStates({
    allSourceResourceNodes: allFlatSourceResourceNodes,
    batchCreateResourceOutput,
    workspaceId: currentWorkspaceId,
    newRootSourceNodeOrder,
  });

  // 6) update resourceDetailsCollection
  updateResourceDetailsCollection({
    allFlatSourceResourceNodes,
    batchCreateResourceOutput,
  });

  // 7) remap resource ids in dockview
  remapOldIdsForDockviewLayout({
    allFlatSourceResourceNodes,
    batchCreateResourceOutput,
    destProjectId: locationTreeNodeData.projectId,
  });

  // 8) reload node paths
  await resourceService.list({
    projectId: locationTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(locationTreeNodeData.parentNode) },
  });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });
};

const createLocationNodes = async ({
  allSourceResourceNodes,
  locationTreeNodeData,
  newRootSourceNodeOrder,
}: {
  allSourceResourceNodes: ResourceNodeWithDetails[];
  locationTreeNodeData: DragResourceNodeData;
  newRootSourceNodeOrder: number;
}) => {
  const sourceResourcesPreparedForCreation = await prepareResourcesForCreation(allSourceResourceNodes);
  const batchCreateResourceInput = await Promise.all(
    sourceResourcesPreparedForCreation.map(async (resource, index) => {
      if (index === 0) {
        return createResourceKind({
          name: resource.name,
          path: locationTreeNodeData.node.path.raw,
          isAddingFolder: resource.kind === "Dir",
          order: newRootSourceNodeOrder,
          protocol: resource.protocol,
          headers: resource.details.headers,
          queryParams: resource.details.queryParams,
          pathParams: resource.details.pathParams,
          body: resource.details.body,
          class: "endpoint",
        });
      } else {
        const newResourcePath = await join(locationTreeNodeData.node.path.raw, resource.path.raw);
        return createResourceKind({
          name: resource.name,
          path: newResourcePath,
          isAddingFolder: resource.kind === "Dir",
          order: -1,
          protocol: resource.details.protocol,
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
