import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { EnvironmentItemState } from "@/workbench/domains/environmentItemState/types";
import {
  CreateEnvironmentInput,
  CreateEnvironmentOutput,
  DeleteEnvironmentInput,
  DeleteEnvironmentOutput,
} from "@repo/ipc";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleCombineProjectEnvToProjectListProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
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

export const handleCombineProjectEnvToProjectList = async ({
  sourceData,
  locationData,
  projectEnvironments,
  currentWorkspaceId,
  batchPutEnvironmentItemState,
  removeEnvironmentItemState,
  deleteEnvironment,
  createEnvironment,
}: HandleCombineProjectEnvToProjectListProps) => {
  const sourceProjectId = sourceData.data.projectId;
  const targetProjectId = locationData.data.projectId;

  if (!sourceProjectId || !targetProjectId) {
    console.error("Project ID not found while combining project environment to project list", {
      sourceProjectId,
      targetProjectId,
    });
    return;
  }

  const sourceProjectEnvs = projectEnvironments.filter((env) => env.projectId === sourceProjectId);
  const targetProjectEnvs = projectEnvironments.filter((env) => env.projectId === targetProjectId);
  const newOrder = targetProjectEnvs.length + 1;

  const newEnvironment = await createEnvironment({
    projectId: targetProjectId,
    name: sourceData.data.name,
    order: newOrder,
    color: sourceData.data.color ?? undefined,
    variables: [],
  });

  const sourceProjectEnvsStatesToUpdate = sourceProjectEnvs
    .filter((env) => env.order > sourceData.data.order && env.id !== sourceData.data.id)
    .map((env) => ({
      id: env.id,
      order: env.order - 1,
    }));

  const allEnvsStatesToUpdate = [
    ...sourceProjectEnvsStatesToUpdate,
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
    projectId: sourceProjectId,
  });
};
