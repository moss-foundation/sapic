import { invokeTauriIpc } from "@/lib/backend/tauri";
import { StreamCollectionsEvent } from "@repo/moss-workspace";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

import { useActiveWorkspace } from "../workspace";

export const USE_STREAM_COLLECTIONS_QUERY_KEY = "streamCollections";

const startStreamCollections = async (): Promise<StreamCollectionsEvent[]> => {
  const collections: StreamCollectionsEvent[] = [];

  const onCollectionEvent = new Channel<StreamCollectionsEvent>();

  onCollectionEvent.onmessage = (collection) => {
    collections.push(collection);
  };

  await invokeTauriIpc("stream_projects", {
    channel: onCollectionEvent,
  });

  return collections;
};

export const useStreamCollections = () => {
  const queryClient = useQueryClient();

  const { hasActiveWorkspace } = useActiveWorkspace();

  const query = useQuery<StreamCollectionsEvent[], Error>({
    queryKey: [USE_STREAM_COLLECTIONS_QUERY_KEY],
    queryFn: async (): Promise<StreamCollectionsEvent[]> => {
      const collections = await startStreamCollections();
      return collections;
    },
    placeholderData: [],
    enabled: hasActiveWorkspace,
  });

  const clearCollectionsCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_STREAM_COLLECTIONS_QUERY_KEY] });
  };

  return {
    ...query,
    clearCollectionsCacheAndRefetch,
  };
};
