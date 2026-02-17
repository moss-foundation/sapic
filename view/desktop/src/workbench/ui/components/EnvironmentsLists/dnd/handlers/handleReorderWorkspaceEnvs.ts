import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { computeSequentialOrders } from "@/utils/computeOrderUpdates";
import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleReorderWorkspaceEnvsProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  workspaceEnvironments: EnvironmentSummary[];
  instruction: Instruction;
  currentWorkspaceId: string;
}

export const handleReorderWorkspaceEnvs = async ({
  sourceData,
  locationData,
  workspaceEnvironments,
  instruction,
  currentWorkspaceId,
}: HandleReorderWorkspaceEnvsProps) => {
  console.log("handleReorderWorkspaceEnvs", {
    sourceData,
    locationData,
    workspaceEnvironments,
    instruction,
    currentWorkspaceId,
  });
  const targetIndex = workspaceEnvironments.findIndex((env) => env.id === locationData.data.id);
  const dropOrder = instruction.operation === "reorder-before" ? targetIndex : targetIndex + 1;

  const reorderedEnvs = [
    ...workspaceEnvironments.slice(0, dropOrder).filter((env) => env.id !== sourceData.data.id),
    sourceData.data,
    ...workspaceEnvironments.slice(dropOrder).filter((env) => env.id !== sourceData.data.id),
  ];

  const updates = computeSequentialOrders(reorderedEnvs);
  if (Object.keys(updates).length === 0) return;

  await environmentItemStateService.batchPutOrder(updates, currentWorkspaceId);
};
