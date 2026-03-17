import { USE_LIST_PROJECT_ENVIRONMENTS_QUERY_KEY } from "@/adapters";
import { environmentSummariesCollection } from "@/db/environmentsSummaries/environmentSummaries";
import { useGetProjectEnvironmentsByProjectId } from "@/db/environmentsSummaries/hooks/useGetProjectEnvironmentsByProjectId";
import { useGetResourcesSummariesByProjectId } from "@/db/resourceSummaries/hooks/useGetResourcesSummariesByProjectId";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { resourceService } from "@/domains/resource/resourceService";
import { useCurrentWorkspace } from "@/hooks";
import { useQueryClient } from "@tanstack/react-query";

export const useRefreshProject = (projectId: string) => {
  const queryClient = useQueryClient();
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: localResourceSummaries } = useGetResourcesSummariesByProjectId(projectId);

  const { data: localEnvironmentSummaries } = useGetProjectEnvironmentsByProjectId(projectId);

  const refreshProject = async () => {
    localResourceSummaries.forEach((resource) => {
      resourceSummariesCollection.delete(resource.id);
    });

    localEnvironmentSummaries?.forEach((env) => {
      environmentSummariesCollection.delete(env.id);
    });

    await Promise.all([
      resourceService.list({ projectId, mode: { RELOAD_PATH: "" } }),
      queryClient.resetQueries({
        queryKey: [USE_LIST_PROJECT_ENVIRONMENTS_QUERY_KEY, currentWorkspaceId, projectId],
      }),
    ]);
  };

  return { refreshProject };
};
