import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { BatchCreateResourceOutput } from "@repo/moss-project";

import { ResourceNode } from "../../types";

export const assignSourceNodesStatesToLocationNodesStates = async ({
  allSourceResourceNodes,
  batchCreateResourceOutput,
  workspaceId,
  newRootSourceNodeOrder,
}: {
  allSourceResourceNodes: ResourceNode[];
  batchCreateResourceOutput: BatchCreateResourceOutput;
  workspaceId: string;
  newRootSourceNodeOrder: number;
}) => {
  const orderMap = new Map<string, number>();
  const expandedMap = new Map<string, boolean>();

  batchCreateResourceOutput.resources.forEach((resource, index) => {
    const sourceResourceNode = allSourceResourceNodes[index];
    if (!sourceResourceNode) return;

    if (sourceResourceNode.order) {
      orderMap.set(resource.id, index === 0 ? newRootSourceNodeOrder : sourceResourceNode.order);
    }
    if (sourceResourceNode.expanded) {
      expandedMap.set(resource.id, sourceResourceNode.expanded);
    }
  });

  if (orderMap.size > 0) {
    await treeItemStateService.batchPutOrder(Object.fromEntries(orderMap), workspaceId);
  }

  if (expandedMap.size > 0) {
    await treeItemStateService.batchPutExpanded(Object.fromEntries(expandedMap), workspaceId);
  }
};
