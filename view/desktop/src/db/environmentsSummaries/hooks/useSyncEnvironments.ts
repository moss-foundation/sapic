import { useEffect } from "react";

import { useListWorkspaceEnvironments } from "@/adapters/tanstackQuery/environment/useListWorkspaceEnvironments";
import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";
import { environmentService } from "@/domains/environment/environmentService";
import { useCurrentWorkspace } from "@/hooks";
import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { ListEnvironmentItem } from "@repo/ipc";

import { environmentSummariesCollection } from "../environmentSummaries";
import { EnvironmentSummary } from "../types";

export const useSyncEnvironments = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projects, isLoading: isProjectsLoading } = useListProjects();
  const { data: workspaceEnvironments, isLoading: isWorkspaceEnvironmentsLoading } = useListWorkspaceEnvironments();

  useEffect(() => {
    if (isProjectsLoading || isWorkspaceEnvironmentsLoading) return;

    const insertEnvironments = async () => {
      if (!workspaceEnvironments || !projects) return;

      environmentSummariesCollection.forEach((env) => {
        environmentSummariesCollection.delete(env.id);
      });

      const projectEnvironments: ListEnvironmentItem[] = [];
      //get project environments
      for await (const project of projects.items) {
        const listProjectEnvironmentsOutput = await environmentService.listProjectEnvironments({
          projectId: project.id,
        });
        projectEnvironments.push(
          ...listProjectEnvironmentsOutput.items.map((env) => ({
            ...env,
            projectId: project.id,
          }))
        );
      }

      //get all environments states
      const envStates = await environmentItemStateService.batchGet(
        [...workspaceEnvironments.items, ...projectEnvironments].map((env) => env.id),
        currentWorkspaceId
      );

      //TODO make it more readable
      const allEnvironments: EnvironmentSummary[] = [
        ...workspaceEnvironments.items.map((env) => ({
          ...env,
          projectId: undefined,
          order: -1,
        })),
        ...projectEnvironments.map((env) => ({
          ...env,
          order: -1,
        })),
      ];

      //all environments
      allEnvironments.forEach((env) => {
        const envState = envStates.find((state) => state.id === env.id);

        if (!environmentSummariesCollection.has(env.id)) {
          environmentSummariesCollection.insert({
            id: env.id,
            projectId: env.projectId ?? undefined,
            isActive: env.isActive,
            name: env.name,
            color: env.color,
            totalVariables: env.totalVariables,

            order: envState?.order ?? -1,
          });
        }
      });
    };

    insertEnvironments();
  }, [currentWorkspaceId, isProjectsLoading, isWorkspaceEnvironmentsLoading, projects, workspaceEnvironments]);
};
