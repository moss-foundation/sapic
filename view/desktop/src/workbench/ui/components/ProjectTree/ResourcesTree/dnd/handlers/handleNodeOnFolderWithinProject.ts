import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { UpdateResourceInput } from "@repo/moss-project";

import { resolveParentPath } from "../handlerOperations/path";
import { updatePeerSourceNodesOrders } from "../handlerOperations/updatePeerSourceNodesOrders";
import { DragResourceNodeData } from "../types.dnd";

interface HandleNodeOnFolderWithinProjectProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DragResourceNodeData;
  locationTreeNodeData: DragResourceNodeData;
}

export const handleNodeOnFolderWithinProject = async ({
  currentWorkspaceId,
  sourceTreeNodeData,
  locationTreeNodeData,
}: HandleNodeOnFolderWithinProjectProps) => {
  //1) update source node path (we don't update all nested nodes because the backend will update them by itself. Calling batchUpdate will cause an error, it will try to update the same node twice)
  await updateSourceNodePath({
    sourceTreeNodeData,
    locationTreeNodeData,
  });

  //2) reload node path(reloading the path here to avoid flickering, because otherwise we update orders first and than the structure)
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });
  await resourceService.list({
    projectId: locationTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(locationTreeNodeData.parentNode) },
  });

  //3) update peer source nodes orders
  await updatePeerSourceNodesOrders({
    sourceNodes: sourceTreeNodeData.parentNode.childNodes,
    deletedNode: sourceTreeNodeData.node,
    workspaceId: currentWorkspaceId,
  });

  //4) update root source node order
  await updateRootSourceNodeOrder({
    locationTreeNodeData,
    sourceTreeNodeData,
    workspaceId: currentWorkspaceId,
  });
};

const updateSourceNodePath = async ({
  sourceTreeNodeData,
  locationTreeNodeData,
}: {
  sourceTreeNodeData: DragResourceNodeData;
  locationTreeNodeData: DragResourceNodeData;
}) => {
  const updatePayload: UpdateResourceInput =
    sourceTreeNodeData.node.kind === "Dir"
      ? {
          DIR: {
            id: sourceTreeNodeData.node.id,
            path: locationTreeNodeData.node.path.raw,
          },
        }
      : {
          ITEM: {
            id: sourceTreeNodeData.node.id,
            path: locationTreeNodeData.node.path.raw,
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: [],
            pathParamsToRemove: [],
            queryParamsToAdd: [],
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
          },
        };

  await resourceService.update(sourceTreeNodeData.projectId, updatePayload);
};

const updateRootSourceNodeOrder = async ({
  locationTreeNodeData,
  sourceTreeNodeData,
  workspaceId,
}: {
  locationTreeNodeData: DragResourceNodeData;
  sourceTreeNodeData: DragResourceNodeData;
  workspaceId: string;
}) => {
  const newOrder = locationTreeNodeData.node.childNodes.length + 1;

  await treeItemStateService.putOrder(sourceTreeNodeData.node.id, newOrder, workspaceId);
};
