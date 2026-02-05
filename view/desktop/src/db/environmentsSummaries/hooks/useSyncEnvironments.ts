import { useEffect } from "react";

import { useStreamEnvironments, useStreamProjects } from "@/adapters";
import { environmentService } from "@/domains/environment/environmentService";
import { useCurrentWorkspace } from "@/hooks";
import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";

import { environmentSummariesCollection } from "../environmentSummaries";

export const useSyncEnvironments = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projects } = useStreamProjects();
  const { data: workspaceEnvironments } = useStreamEnvironments();

  useEffect(() => {
    const insertEnvironments = async () => {
      if (!workspaceEnvironments || !projects) return;

      const allEnvironments = [...workspaceEnvironments];

      //get project environments
      for await (const project of projects) {
        const projectEnvironments = await environmentService.streamProjectEnvironments({
          projectId: project.id,
        });
        allEnvironments.push(...projectEnvironments);
      }

      //get all environments states
      const envStates = await environmentItemStateService.batchGet(
        allEnvironments.map((env) => env.id),
        currentWorkspaceId
      );

      //all environments
      allEnvironments.forEach((env) => {
        const envState = envStates.find((state) => state.id === env.id);

        if (environmentSummariesCollection.has(env.id)) {
          environmentSummariesCollection.update(env.id, (draft) => {
            draft.order = envState?.order ?? -1;
          });
        } else {
          environmentSummariesCollection.insert({
            id: env.id,
            projectId: env.projectId,
            isActive: env.isActive,
            name: env.name,
            color: env.color,
            totalVariables: env.totalVariables,

            order: envState?.order ?? -1,

            metadata: {
              isDirty: false,
            },
          });
        }
      });
    };

    insertEnvironments();
  }, [currentWorkspaceId, projects, workspaceEnvironments]);
};
