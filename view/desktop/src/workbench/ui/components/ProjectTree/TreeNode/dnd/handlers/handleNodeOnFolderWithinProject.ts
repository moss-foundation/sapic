import { UseBatchUpdateProjectResourceInput } from "@/adapters";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { ListProjectResourcesOutput } from "@repo/ipc";
import { BatchUpdateResourceOutput } from "@repo/moss-project";
import { UseMutateAsyncFunction } from "@tanstack/react-query";

import { DragNode, DropNode } from "../../../types";
import {
  makeDirUpdatePayload,
  makeItemUpdatePayload,
  resolveParentPath,
  siblingsAfterRemovalPayload,
} from "../../../utils";

interface HandleNodeOnFolderWithinProjectProps {
  batchUpdateProjectResource: UseMutateAsyncFunction<
    BatchUpdateResourceOutput,
    Error,
    UseBatchUpdateProjectResourceInput,
    unknown
  >;
  currentWorkspaceId: string;
  fetchResourcesForPath: (projectId: string, path: string) => Promise<ListProjectResourcesOutput>;
}

export const handleNodeOnFolderWithinProject =
  ({ batchUpdateProjectResource, currentWorkspaceId, fetchResourcesForPath }: HandleNodeOnFolderWithinProjectProps) =>
  async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode) => {
    const newOrder = locationTreeNodeData.node.childNodes.length + 1;

    const sourceNodeUpdate =
      sourceTreeNodeData.node.kind === "Dir"
        ? makeDirUpdatePayload({
            id: sourceTreeNodeData.node.id,
            path: locationTreeNodeData.node.path.raw,
            order: newOrder,
          })
        : makeItemUpdatePayload({
            id: sourceTreeNodeData.node.id,
            path: locationTreeNodeData.node.path.raw,
            order: newOrder,
          });

    const sourceParentNodes = sourceTreeNodeData.parentNode.childNodes;
    const nodesToUpdate = siblingsAfterRemovalPayload({
      nodes: sourceParentNodes,
      removedNode: sourceTreeNodeData.node,
    });

    const allUpdates = [sourceNodeUpdate, ...nodesToUpdate];
    await batchUpdateProjectResource({
      projectId: sourceTreeNodeData.projectId,
      resources: {
        resources: allUpdates,
      },
    });

    await treeItemStateService.putOrder(sourceTreeNodeData.node.id, newOrder, currentWorkspaceId);
    await treeItemStateService.putExpanded(
      sourceTreeNodeData.node.id,
      sourceTreeNodeData.node.expanded,
      currentWorkspaceId
    );

    await fetchResourcesForPath(locationTreeNodeData.projectId, resolveParentPath(locationTreeNodeData.parentNode));
    await fetchResourcesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));
  };
