import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { computeSequentialOrders } from "@/utils/computeOrderUpdates";
import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleReorderProjectEnvsProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  projectEnvironments: EnvironmentSummary[];
  currentWorkspaceId: string;
  instruction: Instruction;
}

export const handleReorderProjectEnvs = async ({
  sourceData,
  locationData,
  projectEnvironments,
  instruction,
  currentWorkspaceId,
}: HandleReorderProjectEnvsProps) => {
  const projectId = sourceData.data.projectId;
  if (!projectId) {
    console.error("Project ID not found while reordering project environments", { sourceData });
    return;
  }

  const projectEnvironmentsByProjectId = projectEnvironments.filter((env) => env.projectId === projectId);

  const targetIndex = projectEnvironmentsByProjectId.findIndex((env) => env.id === locationData.data.id);
  const dropOrder = instruction.operation === "reorder-before" ? targetIndex : targetIndex + 1;

  const reorderedEnvs = [
    ...projectEnvironmentsByProjectId.slice(0, dropOrder).filter((env) => env.id !== sourceData.data.id),
    sourceData.data,
    ...projectEnvironmentsByProjectId.slice(dropOrder).filter((env) => env.id !== sourceData.data.id),
  ];

  const updates = computeSequentialOrders(reorderedEnvs);
  if (Object.keys(updates).length === 0) return;

  await environmentItemStateService.batchPutOrder(updates, currentWorkspaceId);
};
