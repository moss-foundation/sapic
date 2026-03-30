import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { UpdateResourceInput } from "@repo/moss-project";

import { resolveParentPath } from "../handlerOperations/path";
import { updatePeerSourceNodesOrders } from "../handlerOperations/updatePeerSourceNodesOrders";
import { DragResourceNodeData, LocationResourcesListData } from "../types.dnd";

interface HandleNodeOnListRootWithinProjectProps {
  currentWorkspaceId: string;
  sourceTreeNodeData: DragResourceNodeData;
  locationResourcesListData: LocationResourcesListData;
}

export const handleNodeOnListRootWithinProject = async ({
  currentWorkspaceId,
  sourceTreeNodeData,
  locationResourcesListData,
}: HandleNodeOnListRootWithinProjectProps) => {
  //1) update source node path (we don't update all nested nodes because the backend will update them by itself. Calling batchUpdate will cause an error, it will try to update the same node twice)
  await updateSourceNodePath({ sourceTreeNodeData });

  //2) update peer source nodes orders
  await updatePeerSourceNodesOrders({
    sourceNodes: sourceTreeNodeData.parentNode.childNodes,
    deletedNode: sourceTreeNodeData.node,
    workspaceId: currentWorkspaceId,
  });

  //3) update root source node order
  await updateRootSourceNodeOrder({
    locationResourcesListData,
    sourceTreeNodeData,
    workspaceId: currentWorkspaceId,
  });

  //4) reload node path
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": "" },
  });
};

const updateSourceNodePath = async ({ sourceTreeNodeData }: { sourceTreeNodeData: DragResourceNodeData }) => {
  const updatePayload: UpdateResourceInput =
    sourceTreeNodeData.node.kind === "Dir"
      ? {
          DIR: {
            id: sourceTreeNodeData.node.id,
            path: "",
          },
        }
      : {
          ITEM: {
            id: sourceTreeNodeData.node.id,
            path: "",
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
  locationResourcesListData,
  sourceTreeNodeData,
  workspaceId,
}: {
  locationResourcesListData: LocationResourcesListData;
  sourceTreeNodeData: DragResourceNodeData;
  workspaceId: string;
}) => {
  const newOrder = locationResourcesListData.data.rootResourcesNodes.length + 1;

  await treeItemStateService.putOrder(sourceTreeNodeData.node.id, newOrder, workspaceId);
};
