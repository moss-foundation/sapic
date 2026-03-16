import { environmentService } from "@/domains/environment/environmentService";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY = "listWorkspaceEnvironments";

export const useListWorkspaceEnvironments = () => {
  return useQuery({
    queryKey: [USE_LIST_WORKSPACE_ENVIRONMENTS_QUERY_KEY],
    queryFn: environmentService.listWorkspaceEnvironments,
  });
};
