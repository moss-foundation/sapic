import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { computeOrderUpdates, computeSequentialOrders } from "@/utils/computeOrderUpdates";
import { environmentItemStateService } from "@/workbench/services/environmentItemStateService";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import {
  CreateEnvironmentInput,
  CreateEnvironmentOutput,
  DeleteEnvironmentInput,
  DeleteEnvironmentOutput,
} from "@repo/ipc";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleMoveWorkspaceEnvToProjectEnvsProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  workspaceEnvironments: EnvironmentSummary[];
  projectEnvironments: EnvironmentSummary[];
  instruction: Instruction;
  currentWorkspaceId: string;
  deleteEnvironment: (props: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
  createEnvironment: (props: CreateEnvironmentInput) => Promise<CreateEnvironmentOutput>;
}

export const handleMoveWorkspaceEnvToProjectEnvs = async ({
  sourceData,
  locationData,
  workspaceEnvironments,
  projectEnvironments,
  instruction,
  currentWorkspaceId,
  deleteEnvironment,
  createEnvironment,
}: HandleMoveWorkspaceEnvToProjectEnvsProps) => {
  const projectId = locationData.data.projectId;
  if (!projectId) {
    console.error("Project ID not found while moving workspace environment to project environments", { locationData });
    return;
  }

  const projectEnvironmentsByProjectId = projectEnvironments.filter((env) => env.projectId === projectId);
  const targetIndex = projectEnvironmentsByProjectId.findIndex((env) => env.id === locationData.data.id);
  const dropOrder = instruction.operation === "reorder-before" ? targetIndex : targetIndex + 1;

  const newEnvironment = await createEnvironment({
    projectId,
    name: sourceData.data.name,
    color: sourceData.data.color ?? undefined,
    variables: [],
  });

  const reorderedProjectEnvs = [
    ...projectEnvironmentsByProjectId.slice(0, dropOrder),
    newEnvironment,
    ...projectEnvironmentsByProjectId.slice(dropOrder),
  ];
  const projectUpdates = computeSequentialOrders(reorderedProjectEnvs);

  const remainingWorkspaceEnvs = workspaceEnvironments.filter((env) => env.id !== sourceData.data.id);
  const workspaceUpdates = computeOrderUpdates(remainingWorkspaceEnvs);

  const allUpdates = { ...projectUpdates, ...workspaceUpdates };

  if (Object.keys(allUpdates).length > 0) {
    await environmentItemStateService.batchPutOrder(allUpdates, currentWorkspaceId);
  }

  await environmentItemStateService.removeOrder(sourceData.data.id, currentWorkspaceId);
  await deleteEnvironment({ id: sourceData.data.id });
};
