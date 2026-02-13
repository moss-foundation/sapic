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

interface HandleMoveProjectEnvToWorkspaceEnvsProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  projectEnvironments: EnvironmentSummary[];
  workspaceEnvironments: EnvironmentSummary[];
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

export const handleMoveProjectEnvToWorkspaceEnvs = async ({
  sourceData,
  locationData,
  projectEnvironments,
  workspaceEnvironments,
  instruction,
  currentWorkspaceId,
  batchPutEnvironmentItemState,
  removeEnvironmentItemState,
  deleteEnvironment,
  createEnvironment,
}: HandleMoveProjectEnvToWorkspaceEnvsProps) => {
  const { projectId } = sourceData.data;
  if (!projectId) {
    console.error("Project ID not found while moving project environment to workspace environments", { locationData });
    return;
  }

  const projectEnvs = projectEnvironments.filter((env) => env.projectId === projectId);
  const targetIndex = workspaceEnvironments.find((env) => env.id === locationData.data.id);
  const dropOrder =
    instruction.operation === "reorder-before" ? (targetIndex?.order ?? 0) : (targetIndex?.order ?? 0) + 1;

  const newEnvironment = await createEnvironment({
    name: sourceData.data.name,
    color: sourceData.data.color ?? undefined,
    variables: [],
  });

  const workspaceEnvsStatesToUpdate = workspaceEnvironments
    .filter((env) => env.order >= dropOrder)
    .map((env) => ({
      id: env.id,
      order: env.order + 1,
    }));

  const projectEnvsStatesToUpdate = projectEnvs
    .filter((env) => env.order > sourceData.data.order)
    .map((env) => ({
      id: env.id,
      order: env.order - 1,
    }));

  const allEnvsStatesToUpdate = [
    ...projectEnvsStatesToUpdate,
    ...workspaceEnvsStatesToUpdate,
    {
      id: newEnvironment.id,
      order: dropOrder,
    },
  ];

  await removeEnvironmentItemState({
    id: sourceData.data.id,
    workspaceId: currentWorkspaceId,
  });

  await batchPutEnvironmentItemState({
    environmentItemStates: allEnvsStatesToUpdate,
    workspaceId: currentWorkspaceId,
  });

  await deleteEnvironment({
    id: sourceData.data.id,
    projectId,
  });
};
