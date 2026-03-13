import { flushEnvironmentSummaries } from "@/db/environmentsSummaries/actions/flushEnvironmentSummaries";
import { environmentService } from "@/domains/environment/environmentService";
import { useCurrentWorkspace } from "@/hooks";
import { ListEnvironmentItem } from "@repo/ipc";
import { useQueries, useQueryClient } from "@tanstack/react-query";

import { useListProjects } from "../../project/useListProjects";
import { USE_LIST_PROJECT_ENVIRONMENTS_QUERY_KEY } from "../useListProjectEnvironments";
import { USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY } from "../useListWorkspaceEnvironments";

export const useAllEnvironments = () => {
  const queryClient = useQueryClient();
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: projects, isPending: isProjectsPending } = useListProjects();

  const projectItems = projects?.items ?? [];

  const refreshAllEnvironments = () => {
    flushEnvironmentSummaries();

    queryClient.removeQueries({ queryKey: [USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY] });
    queryClient.removeQueries({ queryKey: [USE_LIST_PROJECT_ENVIRONMENTS_QUERY_KEY] });
  };

  const queries = useQueries({
    queries: [
      {
        queryKey: [USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY, currentWorkspaceId],
        queryFn: () => environmentService.listWorkspaceEnvironments(),
      },
      ...projectItems.map((project) => ({
        queryKey: [USE_LIST_PROJECT_ENVIRONMENTS_QUERY_KEY, currentWorkspaceId, project.id],
        queryFn: () => environmentService.listProjectEnvironments({ projectId: project.id }),
      })),
    ],
    combine: (results) => {
      const [workspaceResult, ...projectResults] = results;

      const workspaceEnvs =
        (workspaceResult?.data as { items?: ListEnvironmentItem[] } | undefined)?.items?.map((env) => ({
          ...env,
          projectId: undefined,
        })) ?? [];

      const projectEnvs = projectResults.flatMap((r, index) => {
        const data = r.data as { items?: ListEnvironmentItem[] } | undefined;
        return data?.items?.map((env) => ({ ...env, projectId: projectItems[index].id })) ?? [];
      });

      return {
        data: [...workspaceEnvs, ...projectEnvs],
        isPending: isProjectsPending || results.some((r) => r.isPending),
      };
    },
  });

  return {
    ...queries,
    refreshAllEnvironments,
  };
};
