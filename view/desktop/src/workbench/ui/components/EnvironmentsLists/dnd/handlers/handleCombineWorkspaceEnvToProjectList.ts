import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { computeOrderUpdates } from "@/utils/computeOrderUpdates";
import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
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
  deleteEnvironment: (props: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
  createEnvironment: (props: CreateEnvironmentInput) => Promise<CreateEnvironmentOutput>;
}

export const handleCombineWorkspaceEnvToProjectList = async ({
  sourceData,
  locationData,
  workspaceEnvironments,
  projectEnvironments,
  currentWorkspaceId,
  deleteEnvironment,
  createEnvironment,
}: HandleCombineWorkspaceEnvToProjectListProps) => {
  const projectId = locationData.data.projectId;
  if (!projectId) {
    console.error("Project ID not found while combining workspace environment to project list", { locationData });
    return;
  }

  const projectEnvironmentsByProjectId = projectEnvironments.filter((env) => env.projectId === projectId);

  const newEnvironment = await createEnvironment({
    projectId,
    name: sourceData.data.name,
    color: sourceData.data.color ?? undefined,
    variables: [],
  });

  const targetEnvsWithNew = [...projectEnvironmentsByProjectId, { id: newEnvironment.id, name: sourceData.data.name }];
  const targetUpdates = computeOrderUpdates(targetEnvsWithNew);

  const remainingWorkspaceEnvs = workspaceEnvironments.filter((env) => env.id !== sourceData.data.id);
  const sourceUpdates = computeOrderUpdates(remainingWorkspaceEnvs);

  const allUpdates = { ...targetUpdates, ...sourceUpdates };

  if (Object.keys(allUpdates).length > 0) {
    await environmentItemStateService.batchPutOrder(allUpdates, currentWorkspaceId);
  }

  await environmentItemStateService.removeOrder(sourceData.data.id, currentWorkspaceId);
  await deleteEnvironment({ id: sourceData.data.id });
};
