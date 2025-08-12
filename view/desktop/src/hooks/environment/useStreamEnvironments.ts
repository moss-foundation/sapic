import { useMemo } from "react";

import { sortByOrder } from "@/components/CollectionTree/utils";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

import { useActiveWorkspace } from "../workspace";

export const USE_STREAMED_ENVIRONMENTS_QUERY_KEY = "streamedEnvironments";

const startStreamingEnvironments = async (): Promise<StreamEnvironmentsEvent[]> => {
  const environments: StreamEnvironmentsEvent[] = [];

  const onEnvironmentEvent = new Channel<StreamEnvironmentsEvent>();

  onEnvironmentEvent.onmessage = (environment) => {
    environments.push(environment);
  };

  await invokeTauriIpc("stream_environments", {
    channel: onEnvironmentEvent,
  });

  return environments;
};

export const useStreamEnvironments = () => {
  const queryClient = useQueryClient();

  const { hasActiveWorkspace } = useActiveWorkspace();

  const query = useQuery<StreamEnvironmentsEvent[], Error>({
    queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY],
    queryFn: startStreamingEnvironments,
    placeholderData: [],
    enabled: hasActiveWorkspace,
  });

  const clearEnvironmentsCacheAndRefetch = () => {
    queryClient.invalidateQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
    queryClient.removeQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
  };

  const environmentsSortedByOrder = useMemo(() => {
    return sortByOrder(query.data ?? []);
  }, [query.data]);

  return {
    ...query,
    clearEnvironmentsCacheAndRefetch,
    environmentsSortedByOrder,
  };
};
