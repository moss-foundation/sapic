import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { CreateEnvironmentParams } from "@/domains/environment/environmentService";
import { computeOrderUpdates } from "@/utils/computeOrderUpdates";
import { environmentItemStateService } from "@/workbench/services/environmentItemStateService";
import { CreateEnvironmentOutput, DeleteEnvironmentInput, DeleteEnvironmentOutput } from "@repo/ipc";

import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleCombineProjectEnvToProjectListProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  projectEnvironments: EnvironmentSummary[];
  currentWorkspaceId: string;
  deleteEnvironment: (props: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
  createEnvironment: (props: CreateEnvironmentParams) => Promise<CreateEnvironmentOutput>;
}

export const handleCombineProjectEnvToProjectList = async ({
  sourceData,
  locationData,
  projectEnvironments,
  currentWorkspaceId,
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

  const newEnvironment = await createEnvironment({
    projectId: targetProjectId,
    name: sourceData.data.name,
    color: sourceData.data.color ?? undefined,
    variables: [],
    order: targetProjectEnvs.length + 1,
    expanded: sourceData.data.expanded,
  });

  const remainingSourceEnvs = sourceProjectEnvs.filter((env) => env.id !== sourceData.data.id);
  const sourceUpdates = computeOrderUpdates(remainingSourceEnvs);

  const targetEnvsWithNew = [...targetProjectEnvs, { id: newEnvironment.id, name: sourceData.data.name }];
  const targetUpdates = computeOrderUpdates(targetEnvsWithNew);

  const allUpdates = { ...sourceUpdates, ...targetUpdates };

  if (Object.keys(allUpdates).length > 0) {
    await environmentItemStateService.batchPutOrder(allUpdates, currentWorkspaceId);
  }

  await environmentItemStateService.removeOrder(sourceData.data.id, currentWorkspaceId);
  await deleteEnvironment({ id: sourceData.data.id, projectId: sourceProjectId });
};
