import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { CreateEnvironmentParams } from "@/domains/environment/environmentService";
import { computeOrderUpdates } from "@/utils/computeOrderUpdates";
import { environmentItemStateService } from "@/workbench/services/environmentItemStateService";
import { CreateEnvironmentOutput, DeleteEnvironmentInput, DeleteEnvironmentOutput } from "@repo/ipc";

import { remapEnvironmentIdInDockviewLayout } from "../handlerOperations/remapEnvironmentIdInSerializedDockview";
import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

interface HandleCombineProjectEnvToWorkspaceListProps {
  sourceData: DragEnvironmentItem;
  locationData: DropEnvironmentItem;
  projectEnvironments: EnvironmentSummary[];
  workspaceEnvironments: EnvironmentSummary[];
  currentWorkspaceId: string;
  deleteEnvironment: (props: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
  createEnvironment: (props: CreateEnvironmentParams) => Promise<CreateEnvironmentOutput>;
}

export const handleCombineProjectEnvToWorkspaceList = async ({
  sourceData,
  locationData,
  projectEnvironments,
  workspaceEnvironments,
  currentWorkspaceId,
  deleteEnvironment,
  createEnvironment,
}: HandleCombineProjectEnvToWorkspaceListProps) => {
  const sourceProjectId = sourceData.data.projectId;

  if (!sourceProjectId) {
    console.error("Project ID not found while combining project environment to workspace list", { locationData });
    return;
  }

  const sourceProjectEnvs = projectEnvironments.filter((env) => env.projectId === sourceProjectId);

  const newEnvironment = await createEnvironment({
    name: sourceData.data.name,
    color: sourceData.data.color ?? undefined,
    variables: [],
    order: workspaceEnvironments.length + 1,
    expanded: sourceData.data.expanded,
  });

  const remainingSourceEnvs = sourceProjectEnvs.filter((env) => env.id !== sourceData.data.id);
  const sourceUpdates = computeOrderUpdates(remainingSourceEnvs);

  const targetEnvsWithNew = [...workspaceEnvironments, { id: newEnvironment.id, name: sourceData.data.name }];
  const targetUpdates = computeOrderUpdates(targetEnvsWithNew);

  const allUpdates = { ...sourceUpdates, ...targetUpdates };

  if (Object.keys(allUpdates).length > 0) {
    await environmentItemStateService.batchPutOrder(allUpdates, currentWorkspaceId);
  }

  await environmentItemStateService.removeOrder(sourceData.data.id, currentWorkspaceId);
  await deleteEnvironment({ id: sourceData.data.id, projectId: sourceProjectId });

  remapEnvironmentIdInDockviewLayout({
    oldEnvId: sourceData.data.id,
    newEnvId: newEnvironment.id,
    newTabIcon: "WorkspaceEnvironment",
  });
};
