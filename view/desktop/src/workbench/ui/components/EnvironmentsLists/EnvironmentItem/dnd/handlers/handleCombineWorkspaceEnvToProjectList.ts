import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { EnvironmentItemState } from "@/workbench/domains/environmentItemState/types";
import {
  CreateEnvironmentInput,
  CreateEnvironmentOutput,
  DeleteEnvironmentInput,
  DeleteEnvironmentOutput,
} from "@repo/ipc";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleCombineWorkspaceEnvToProjectListProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  workspaceEnvironments: EnvironmentSummary[];
  projectEnvironments: EnvironmentSummary[];
  currentWorkspaceId: string;
  batchPutEnvironmentItemState: (props: {
    environmentItemStates: EnvironmentItemState[];
    workspaceId: string;
  }) => Promise<void>;
  removeEnvironmentItemState: (props: { id: string; workspaceId: string }) => Promise<void>;
  deleteEnvironment: (props: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
  createEnvironment: (props: CreateEnvironmentInput) => Promise<CreateEnvironmentOutput>;
}

export const handleCombineWorkspaceEnvToProjectList = async ({
  sourceData,
  locationData,
  workspaceEnvironments,
  projectEnvironments,
  currentWorkspaceId,
  batchPutEnvironmentItemState,
  removeEnvironmentItemState,
  deleteEnvironment,
  createEnvironment,
}: HandleCombineWorkspaceEnvToProjectListProps) => {
  const projectId = locationData.data.projectId;
  if (!projectId) {
    console.error("Project ID not found while combining workspace environment to project list", { locationData });
    return;
  }

  const projectEnvironmentsByProjectId = projectEnvironments.filter((env) => env.projectId === projectId);
  const newOrder = projectEnvironmentsByProjectId.length + 1;

  const newEnvironment = await createEnvironment({
    projectId,
    name: sourceData.data.name,
    order: newOrder,
    color: sourceData.data.color ?? undefined,
    variables: [],
  });

  const workspaceEnvsStatesToUpdate =
    workspaceEnvironments
      .filter((env) => env.order! > sourceData.data.order! && env.id !== sourceData.data.id)
      .map((env) => ({
        id: env.id,
        order: env.order! - 1,
      })) ?? [];

  const allEnvsStatesToUpdate = [
    ...workspaceEnvsStatesToUpdate,
    {
      id: newEnvironment.id,
      order: newOrder,
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
  });
};
