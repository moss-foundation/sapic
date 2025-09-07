import { useMemo } from "react";

import { invokeTauriIpc } from "@/lib/backend/tauri";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { StreamEnvironmentsEvent, StreamEnvironmentsOutput } from "@repo/moss-workspace";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

import { useActiveWorkspace } from "..";

export const USE_STREAMED_ENVIRONMENTS_QUERY_KEY = "streamedEnvironments";

export interface StreamEnvironmentsResult {
  environments: StreamEnvironmentsEvent[];
  groups: StreamEnvironmentsOutput["groups"];
}

const startStreamingEnvironments = async (): Promise<StreamEnvironmentsResult> => {
  const environments: StreamEnvironmentsEvent[] = [];

  const onEnvironmentEvent = new Channel<StreamEnvironmentsEvent>();

  onEnvironmentEvent.onmessage = (environment) => {
    environments.push(environment);
  };

  const groups = await invokeTauriIpc<StreamEnvironmentsOutput>("stream_environments", {
    channel: onEnvironmentEvent,
  });

  if (groups.status === "error") {
    throw new Error(String(groups.error));
  }

  console.log({ groups });

  return { environments, groups: groups.data.groups };
};

export const useStreamEnvironments = () => {
  const queryClient = useQueryClient();

  const { hasActiveWorkspace } = useActiveWorkspace();

  const query = useQuery<StreamEnvironmentsResult, Error>({
    queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY],
    queryFn: startStreamingEnvironments,
    placeholderData: { environments: [], groups: [] },
    enabled: hasActiveWorkspace,
  });

  const clearEnvironmentsCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
  };

  const globalEnvironments = useMemo(() => {
    if (!query.data) return [];

    const globalEnvironments = query.data.environments.filter((environment) => !environment.collectionId);

    if (globalEnvironments.length === 0) return [];

    return sortObjectsByOrder(globalEnvironments);
  }, [query.data]);

  const collectionEnvironments = useMemo(() => {
    if (!query.data) return [];

    const collectionEnvironments = query.data.environments.filter((environment) => environment.collectionId);

    if (collectionEnvironments.length === 0) return [];

    return sortObjectsByOrder(collectionEnvironments);
  }, [query.data]);

  const groups = query.data?.groups;

  return {
    ...query,
    clearEnvironmentsCacheAndRefetch,
    globalEnvironments,
    collectionEnvironments,
    groups,
  };
};
