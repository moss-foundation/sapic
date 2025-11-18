import { useMemo } from "react";

import { environmentService } from "@/domains/environment/environmentService";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { useQuery, useQueryClient } from "@tanstack/react-query";

export const USE_STREAMED_ENVIRONMENTS_QUERY_KEY = "streamedEnvironments";

export const useStreamEnvironments = () => {
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY],
    queryFn: environmentService.streamEnvironments,
    placeholderData: { environments: [], groups: [] },
  });

  const clearEnvironmentsCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
  };

  const globalEnvironments = useMemo(() => {
    if (!query.data) return [];

    const globalEnvironments = query.data.environments.filter((environment) => environment.projectId === null);

    if (globalEnvironments.length === 0) return [];

    return sortObjectsByOrder(globalEnvironments);
  }, [query.data]);

  const projectEnvironments = useMemo(() => {
    if (!query.data) return [];

    const projectEnvironments = query.data.environments.filter((environment) => environment.projectId !== null);

    if (projectEnvironments.length === 0) return [];

    return sortObjectsByOrder(projectEnvironments);
  }, [query.data]);

  const groups = sortObjectsByOrder(query.data?.groups ?? []);

  return {
    ...query,
    clearEnvironmentsCacheAndRefetch,
    globalEnvironments,
    projectEnvironments,
    groups,
  };
};
