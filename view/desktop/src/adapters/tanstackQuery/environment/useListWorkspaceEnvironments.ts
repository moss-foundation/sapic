import { environmentSummariesCollection } from "@/db/environmentsSummaries/environmentSummaries";
import { environmentService } from "@/domains/environment/environmentService";
import { useQuery, useQueryClient } from "@tanstack/react-query";

export const USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY = "listWorkspaceEnvironments";

export const useListWorkspaceEnvironments = () => {
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: [USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY],
    queryFn: environmentService.listWorkspaceEnvironments,
  });

  const flushWorkspaceEnvironments = () => {
    environmentSummariesCollection.forEach((environment) => environmentSummariesCollection.delete(environment.id));
    queryClient.removeQueries({ queryKey: [USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY] });
  };

  return {
    ...query,
    flushWorkspaceEnvironments,
  };
};
