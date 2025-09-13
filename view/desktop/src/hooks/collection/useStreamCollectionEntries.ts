import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEntriesEvent } from "@repo/moss-collection";
import { useQuery, useQueryClient } from "@tanstack/react-query";

import { useActiveWorkspace } from "../workspace/derived/useActiveWorkspace";
import { startStreamingCollectionEntries } from "./queries/startStreamingCollectionEntries";

export const USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY = "streamCollectionEntries";

export const useStreamCollectionEntries = (collectionId: string) => {
  const queryClient = useQueryClient();

  const { api } = useTabbedPaneStore();

  const { hasActiveWorkspace } = useActiveWorkspace();

  const query = useQuery<StreamEntriesEvent[], Error>({
    queryKey: [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, collectionId],
    queryFn: async () => {
      const entires = await startStreamingCollectionEntries(collectionId);

      //Remove panels that contain entries that are not in the entries array
      api?.panels.forEach((panel) => {
        if (!panel.params?.collectionId) return;

        if (!entires.some((entry) => entry.id === panel.id)) {
          const panelToRemove = api?.getPanel(panel.id);
          if (panelToRemove) {
            api?.removePanel(panelToRemove);
          }
        }
      });

      return entires;
    },
    placeholderData: [],
    enabled: hasActiveWorkspace,
  });

  const clearEntriesCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY] });
  };

  return {
    ...query,
    clearEntriesCacheAndRefetch,
  };
};
