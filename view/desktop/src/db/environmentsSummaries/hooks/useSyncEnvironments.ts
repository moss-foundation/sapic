import { useEffect } from "react";

import { useListWorkspaceEnvironments } from "@/adapters/tanstackQuery/environment/useListWorkspaceEnvironments";
import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";
import { environmentService } from "@/domains/environment/environmentService";
import { useCurrentWorkspace } from "@/hooks";
import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { ListEnvironmentItem, ListProjectsOutput } from "@repo/ipc";

import { environmentSummariesCollection } from "../environmentSummaries";
import { EnvironmentSummary } from "../types";

type ListEnvironmentItemWithProjectId = ListEnvironmentItem & {
  projectId?: string;
};

export const useSyncEnvironments = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projects, isFetching: isProjectsLoading } = useListProjects();
  const { data: workspaceEnvironments, isFetching: isWorkspaceEnvironmentsLoading } = useListWorkspaceEnvironments();

  useEffect(() => {
    if (isProjectsLoading || isWorkspaceEnvironmentsLoading || !projects || !workspaceEnvironments) return;

    const syncEnvironments = async () => {
      clearExistingEnvironments();

      const projectEnvironments = await fetchAllProjectEnvironments(projects);

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
  }, [currentWorkspaceId, isProjectsLoading, isWorkspaceEnvironmentsLoading, projects, workspaceEnvironments]);
};

const fetchAllProjectEnvironments = async (
  projects: ListProjectsOutput
): Promise<ListEnvironmentItemWithProjectId[]> => {
  const promises = projects.items.map(async (project) => {
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
