import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { EnvironmentItemState } from "@/workbench/domains/environmentItemState/types";
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
  batchPutEnvironmentItemState: (props: {
    environmentItemStates: EnvironmentItemState[];
    workspaceId: string;
  }) => Promise<void>;
  removeEnvironmentItemState: (props: { id: string; workspaceId: string }) => Promise<void>;
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
  batchPutEnvironmentItemState,
  removeEnvironmentItemState,
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
    order: dropOrder,
    color: sourceData.data.color ?? undefined,
    variables: [],
  });

  const projectEnvsStatesToUpdate = [
    ...projectEnvironmentsByProjectId.slice(0, dropOrder),
    newEnvironment,
    ...projectEnvironmentsByProjectId.slice(dropOrder),
  ].map((env, index) => ({
    id: env.id,
    order: index + 1,
  }));

  const workspaceEnvsStatesToUpdate =
    workspaceEnvironments
      .filter((env) => env.order! > sourceData.data.order! && env.id !== sourceData.data.id)
      .map((env) => ({
        id: env.id,
        order: env.order! - 1,
      })) ?? [];

  const allEnvsStatesToUpdate = [...projectEnvsStatesToUpdate, ...workspaceEnvsStatesToUpdate];

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
  });
};
