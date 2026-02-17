import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { computeOrderUpdates, computeSequentialOrders } from "@/utils/computeOrderUpdates";
import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import {
  CreateEnvironmentInput,
  CreateEnvironmentOutput,
  DeleteEnvironmentInput,
  DeleteEnvironmentOutput,
} from "@repo/ipc";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleMoveProjectEnvToWorkspaceEnvsProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  projectEnvironments: EnvironmentSummary[];
  workspaceEnvironments: EnvironmentSummary[];
  instruction: Instruction;
  currentWorkspaceId: string;
  deleteEnvironment: (props: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
  createEnvironment: (props: CreateEnvironmentInput) => Promise<CreateEnvironmentOutput>;
}

export const handleMoveProjectEnvToWorkspaceEnvs = async ({
  sourceData,
  locationData,
  projectEnvironments,
  workspaceEnvironments,
  instruction,
  currentWorkspaceId,
  deleteEnvironment,
  createEnvironment,
}: HandleMoveProjectEnvToWorkspaceEnvsProps) => {
  const { projectId } = sourceData.data;
  if (!projectId) {
    console.error("Project ID not found while moving project environment to workspace environments", { locationData });
    return;
  }

  const projectEnvs = projectEnvironments.filter((env) => env.projectId === projectId);
  const targetIndex = workspaceEnvironments.findIndex((env) => env.id === locationData.data.id);
  const dropOrder = instruction.operation === "reorder-before" ? targetIndex : targetIndex + 1;

  const newEnvironment = await createEnvironment({
    name: sourceData.data.name,
    color: sourceData.data.color ?? undefined,
    variables: [],
  });

  const reorderedWorkspaceEnvs = [
    ...workspaceEnvironments.slice(0, dropOrder),
    newEnvironment,
    ...workspaceEnvironments.slice(dropOrder),
  ];
  const workspaceUpdates = computeSequentialOrders(reorderedWorkspaceEnvs);

  const remainingProjectEnvs = projectEnvs.filter((env) => env.id !== sourceData.data.id);
  const projectUpdates = computeOrderUpdates(remainingProjectEnvs);

  const allUpdates = { ...workspaceUpdates, ...projectUpdates };

  if (Object.keys(allUpdates).length > 0) {
    await environmentItemStateService.batchPutOrder(allUpdates, currentWorkspaceId);
  }

  await environmentItemStateService.removeOrder(sourceData.data.id, currentWorkspaceId);
  await deleteEnvironment({ id: sourceData.data.id, projectId });
};
