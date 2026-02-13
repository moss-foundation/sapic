import { environmentService } from "@/domains/environment/environmentService";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_PROJECT_ENVIRONMENTS_QUERY_KEY = "listProjectEnvironments";

export const useListProjectEnvironments = (projectId: string) => {
  return useQuery({
    queryKey: [USE_LIST_PROJECT_ENVIRONMENTS_QUERY_KEY],
    queryFn: () => environmentService.listProjectEnvironments({ projectId }),
  });
};
