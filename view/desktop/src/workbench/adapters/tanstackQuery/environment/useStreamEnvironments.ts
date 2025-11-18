import { useMemo } from "react";

import { StreamEnvironmentsResult } from "@/domains/environment/ipc";
import { environmentIpc } from "@/infra/ipc/environment";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

export const USE_STREAMED_ENVIRONMENTS_QUERY_KEY = "streamedEnvironments";

const startStreamingEnvironments = async (): Promise<StreamEnvironmentsResult> => {
  const environments: StreamEnvironmentsEvent[] = [];

  const environmentEvent = new Channel<StreamEnvironmentsEvent>();
  environmentEvent.onmessage = (environment) => {
    environments.push(environment);
  };

  const groups = await environmentIpc.streamEnvironments(environmentEvent);

  return { environments, groups: groups.groups };
};

export const useStreamEnvironments = () => {
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY],
    queryFn: startStreamingEnvironments,
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
