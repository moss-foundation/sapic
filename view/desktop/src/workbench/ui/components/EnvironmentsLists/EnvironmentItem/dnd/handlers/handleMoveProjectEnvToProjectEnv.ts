import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { EnvironmentItemState } from "@/workbench/domains/environmentItemState/types";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import {
  CreateEnvironmentInput,
  CreateEnvironmentOutput,
  DeleteEnvironmentInput,
  DeleteEnvironmentOutput,
} from "@repo/ipc";

import { DragEnvironmentItem, DropEnvironmentItem } from "../../types.dnd";

interface HandleMoveProjectEnvToProjectEnvProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  projectEnvironments: EnvironmentSummary[];
  instruction: Instruction;
  currentWorkspaceId: string;
  batchPutEnvironmentItemState: (props: {
    environmentItemStates: EnvironmentItemState[];
    workspaceId: string;
  }) => Promise<void>;
  removeEnvironmentItemState: (props: { id: string; workspaceId: string }) => Promise<void>;
  deleteEnvironment: (props: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
  createEnvironment: (props: CreateEnvironmentInput) => Promise<CreateEnvironmentOutput>;
}

export const handleMoveProjectEnvToProjectEnv = async ({
  sourceData,
  locationData,
  projectEnvironments,
  instruction,
  currentWorkspaceId,
  batchPutEnvironmentItemState,
  removeEnvironmentItemState,
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

  const targetIndex = targetProjectEnvs.find((env) => env.id === locationData.data.id);
  const dropOrder =
    instruction.operation === "reorder-before" ? (targetIndex?.order ?? 0) : (targetIndex?.order ?? 0) + 1;

  const newEnvironment = await createEnvironment({
    projectId: targetProjectId,
    name: sourceData.data.name,
    order: dropOrder,
    variables: [],
  });

  const sourceProjectEnvsStatesToUpdate = sourceProjectEnvs
    .filter((env) => env.order > sourceData.data.order)
    .map((env) => ({
      id: env.id,
      order: env.order - 1,
    }));

  const targetProjectEnvsStatesToUpdate = targetProjectEnvs
    .filter((env) => env.order >= dropOrder)
    .map((env) => ({
      id: env.id,
      order: env.order + 1,
    }));

  const allEnvsStatesToUpdate = [
    ...sourceProjectEnvsStatesToUpdate,
    ...targetProjectEnvsStatesToUpdate,
    {
      id: newEnvironment.id,
      order: dropOrder,
    },
  ];

  await batchPutEnvironmentItemState({
    environmentItemStates: allEnvsStatesToUpdate,
    workspaceId: currentWorkspaceId,
  });

  await removeEnvironmentItemState({
    id: sourceData.data.id,
    workspaceId: currentWorkspaceId,
  });

  await deleteEnvironment({
    id: sourceData.data.id,
    projectId: sourceProjectId,
  });
};
