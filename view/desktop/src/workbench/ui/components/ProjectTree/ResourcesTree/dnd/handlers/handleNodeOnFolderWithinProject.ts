import { UseBatchUpdateProjectResourceInput } from "@/adapters";
import { resourceService } from "@/domains/resource/resourceService";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
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
  sourceTreeNodeData: DragNode;
  locationTreeNodeData: DropNode;
}

export const handleNodeOnFolderWithinProject = async ({
  batchUpdateProjectResource,
  currentWorkspaceId,
  sourceTreeNodeData,
  locationTreeNodeData,
}: HandleNodeOnFolderWithinProjectProps) => {
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

  await resourceService.list({
    projectId: locationTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(locationTreeNodeData.parentNode) },
  });
  await resourceService.list({
    projectId: sourceTreeNodeData.projectId,
    mode: { "RELOAD_PATH": resolveParentPath(sourceTreeNodeData.parentNode) },
  });
};
