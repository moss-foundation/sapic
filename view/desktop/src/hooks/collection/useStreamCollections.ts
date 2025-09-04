import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useTabbedPaneStore } from "@/store/tabbedPane";
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

  const res = await invokeTauriIpc("stream_collections", {
    channel: onCollectionEvent,
  });

  console.log(res);

  return collections;
};

export const useStreamCollections = () => {
  const queryClient = useQueryClient();
  const { api } = useTabbedPaneStore();

  const { hasActiveWorkspace } = useActiveWorkspace();

  const query = useQuery<StreamCollectionsEvent[], Error>({
    queryKey: [USE_STREAM_COLLECTIONS_QUERY_KEY],
    queryFn: async (): Promise<StreamCollectionsEvent[]> => {
      const collections = await startStreamCollections();

      //Remove panels that contain collections that are not in the collections array
      api?.panels.forEach((panel) => {
        if (!panel.params?.collectionId) return;

        if (!collections.some((collection) => collection.id === panel.id)) {
          const panelToRemove = api?.getPanel(panel.id);
          if (panelToRemove) {
            api?.removePanel(panelToRemove);
          }
        }
      });

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
