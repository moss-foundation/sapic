import { invokeTauriIpc } from "@/lib/backend/tauri";
import { StreamCollectionsEvent } from "@repo/moss-workspace";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

import { useWorkspaceSidebarState } from "../workspace/useWorkspaceSidebarState";

export const USE_STREAMED_COLLECTIONS_QUERY_KEY = "streamedCollections";

const startStreamingCollections = async (): Promise<StreamCollectionsEvent[]> => {
  const collections: StreamCollectionsEvent[] = [];

  const onCollectionEvent = new Channel<StreamCollectionsEvent>();

  onCollectionEvent.onmessage = (collection) => {
    collections.push(collection);
  };

  await invokeTauriIpc("stream_collections", {
    channel: onCollectionEvent,
  });

  return collections;
};

export const useStreamedCollections = () => {
  const { hasWorkspace } = useWorkspaceSidebarState();

  const queryClient = useQueryClient();

  const query = useQuery<StreamCollectionsEvent[], Error>({
    queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY],
    queryFn: startStreamingCollections,
    placeholderData: [],
    enabled: hasWorkspace,
  });

  const clearQueryCacheAndRefetch = () => {
    queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY], exact: true });
    queryClient.removeQueries({ queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY], exact: true });
    return query.refetch();
  };

  return {
    ...query,
    clearQueryCacheAndRefetch,
  };
};
