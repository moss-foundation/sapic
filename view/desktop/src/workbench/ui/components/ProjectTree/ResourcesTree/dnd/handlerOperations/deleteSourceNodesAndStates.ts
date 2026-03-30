import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";

import { DragResourceNodeData, ResourceNodeWithDetails } from "../types.dnd";

export const deleteSourceNodesAndStates = async ({
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
