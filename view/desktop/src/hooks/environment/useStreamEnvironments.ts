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

  if (environments.length === 0) {
    return [
      {
        id: "31JQFM9hPz_qqq",
        collectionId: "31JQFM9hPz",
        name: "Env 1",
        order: 1,
        expanded: true,
      },
      {
        id: "u_WMAATtDg_qqq",
        name: "Env 2",
        order: 2,
        expanded: true,
      },
      {
        id: "v2dqXauAIY_qqq",
        collectionId: "v2dqXauAIY",
        name: "Env 3",
        order: 3,
        expanded: true,
      },
      {
        id: "rnr5ynry",
        name: "Env 4",
        order: 4,
        expanded: true,
      },
    ];
  }

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

  return {
    ...query,
    clearEnvironmentsCacheAndRefetch,
  };
};
