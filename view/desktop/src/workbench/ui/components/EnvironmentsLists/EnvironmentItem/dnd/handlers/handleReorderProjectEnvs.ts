import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { EnvironmentItemState } from "@/workbench/domains/environmentItemState/types";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleReorderProjectEnvsProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  projectEnvironments: EnvironmentSummary[];
  currentWorkspaceId: string;
  batchPutEnvironmentItemState: (props: {
    environmentItemStates: EnvironmentItemState[];
    workspaceId: string;
  }) => Promise<void>;
  instruction: Instruction;
}

export const handleReorderProjectEnvs = async ({
  sourceData,
  locationData,
  projectEnvironments,
  batchPutEnvironmentItemState,
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

  const environmentsToUpdate = [
    ...projectEnvironmentsByProjectId.slice(0, dropOrder).filter((env) => env.id !== sourceData.data.id),
    sourceData.data,
    ...projectEnvironmentsByProjectId.slice(dropOrder).filter((env) => env.id !== sourceData.data.id),
  ].map((env, index) => ({
    id: env.id,
    order: index + 1,
  }));

  await batchPutEnvironmentItemState({
    workspaceId: currentWorkspaceId,
    environmentItemStates: environmentsToUpdate.map((env) => ({
      id: env.id,
      order: env.order,
    })),
  });
};
