import { useEffect } from "react";

import { useListWorkspaceEnvironments } from "@/adapters/tanstackQuery/environment/useListWorkspaceEnvironments";
import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { ProjectSummary } from "@/db/projectSummaries/types";
import { environmentService } from "@/domains/environment/environmentService";
import { useCurrentWorkspace } from "@/hooks";
import { environmentItemStateService } from "@/workbench/services/environmentItemStateService";
import { ListEnvironmentItem } from "@repo/ipc";

import { environmentSummariesCollection } from "../environmentSummaries";
import { EnvironmentSummary } from "../types";

type ListEnvironmentItemWithProjectId = ListEnvironmentItem & {
  projectId?: string;
};

export const useSyncEnvironments = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: localProjectSummaries } = useGetAllLocalProjectSummaries();
  const { data: workspaceEnvironments, isFetching: isWorkspaceEnvironmentsLoading } = useListWorkspaceEnvironments();

  useEffect(() => {
    if (isWorkspaceEnvironmentsLoading || !workspaceEnvironments) return;

    const syncEnvironments = async () => {
      const projectEnvironments = await fetchAllProjectEnvironments(localProjectSummaries);

      const allEnvironments: ListEnvironmentItemWithProjectId[] = [
        ...workspaceEnvironments.items.map((env) => ({ ...env, projectId: undefined })),
        ...projectEnvironments,
      ];

      const envIds = allEnvironments.map((env) => env.id);
      const [envOrders, envExpanded] = await Promise.all([
        environmentItemStateService.batchGetOrder(envIds, currentWorkspaceId),
        environmentItemStateService.batchGetExpanded(envIds, currentWorkspaceId),
      ]);

      const summaries: EnvironmentSummary[] = allEnvironments.map((env, index) => ({
        id: env.id,
        projectId: env.projectId,
        isActive: env.isActive,
        name: env.name,
        color: env.color,
        totalVariables: env.totalVariables,
        order: envOrders[index],
        expanded: envExpanded[index] ?? false,
      }));

      insertEnvironmentSummaries(summaries);
    };
    syncEnvironments();
  }, [currentWorkspaceId, isWorkspaceEnvironmentsLoading, localProjectSummaries, workspaceEnvironments]);

  useEffect(() => {
    clearExistingEnvironments();
  }, [currentWorkspaceId]);
};

const fetchAllProjectEnvironments = async (projects: ProjectSummary[]): Promise<ListEnvironmentItemWithProjectId[]> => {
  const promises = projects.map(async (project) => {
    const result = await environmentService.listProjectEnvironments({
      projectId: project.id,
    });

    return result.items.map((env) => ({
      ...env,
      projectId: project.id,
    }));
  });

  const results = await Promise.all(promises);
  return results.flat();
};

const clearExistingEnvironments = () => {
  environmentSummariesCollection.forEach((env) => {
    environmentSummariesCollection.delete(env.id);
  });
};

const insertEnvironmentSummaries = (summaries: EnvironmentSummary[]) => {
  summaries.forEach((summary) => {
    if (!environmentSummariesCollection.has(summary.id)) {
      environmentSummariesCollection.insert(summary);
    }
  });
};
