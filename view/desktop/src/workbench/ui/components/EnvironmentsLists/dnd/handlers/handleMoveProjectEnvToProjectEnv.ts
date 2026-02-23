import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { CreateEnvironmentParams } from "@/domains/environment/environmentService";
import { computeOrderUpdates, computeSequentialOrders } from "@/utils/computeOrderUpdates";
import { environmentItemStateService } from "@/workbench/services/environmentItemStateService";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { CreateEnvironmentOutput, DeleteEnvironmentInput, DeleteEnvironmentOutput } from "@repo/ipc";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleMoveProjectEnvToProjectEnvProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  projectEnvironments: EnvironmentSummary[];
  instruction: Instruction;
  currentWorkspaceId: string;
  deleteEnvironment: (props: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
  createEnvironment: (props: CreateEnvironmentParams) => Promise<CreateEnvironmentOutput>;
}

export const handleMoveProjectEnvToProjectEnv = async ({
  sourceData,
  locationData,
  projectEnvironments,
  instruction,
  currentWorkspaceId,
  deleteEnvironment,
  createEnvironment,
}: HandleMoveProjectEnvToProjectEnvProps) => {
  const sourceProjectId = sourceData.data.projectId;
  const targetProjectId = locationData.data.projectId;

  if (!targetProjectId || !sourceProjectId) {
    console.error("Project ID not found while moving project environment to project environment", {
      sourceProjectId,
      targetProjectId,
    });
    return;
  }

  const sourceProjectEnvs = projectEnvironments.filter((env) => env.projectId === sourceProjectId);
  const targetProjectEnvs = projectEnvironments.filter((env) => env.projectId === targetProjectId);

  const targetIndex = targetProjectEnvs.findIndex((env) => env.id === locationData.data.id);
  const dropOrder = instruction.operation === "reorder-before" ? targetIndex : targetIndex + 1;

  const newEnvironment = await createEnvironment({
    projectId: targetProjectId,
    name: sourceData.data.name,
    variables: [],
    order: dropOrder,
    expanded: sourceData.data.expanded,
  });

  const remainingSourceEnvs = sourceProjectEnvs.filter((env) => env.id !== sourceData.data.id);
  const sourceUpdates = computeOrderUpdates(remainingSourceEnvs);

  const reorderedTargetEnvs = [
    ...targetProjectEnvs.slice(0, dropOrder),
    newEnvironment,
    ...targetProjectEnvs.slice(dropOrder),
  ];
  const targetUpdates = computeSequentialOrders(reorderedTargetEnvs);

  const allUpdates = { ...sourceUpdates, ...targetUpdates };

  if (Object.keys(allUpdates).length > 0) {
    await environmentItemStateService.batchPutOrder(allUpdates, currentWorkspaceId);
  }

  await environmentItemStateService.removeOrder(sourceData.data.id, currentWorkspaceId);
  await deleteEnvironment({ id: sourceData.data.id, projectId: sourceProjectId });
};
