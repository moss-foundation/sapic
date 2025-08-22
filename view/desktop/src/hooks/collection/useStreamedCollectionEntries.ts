import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEntriesEvent } from "@repo/moss-collection";
import { useQuery, useQueryClient } from "@tanstack/react-query";

import { useActiveWorkspace } from "../workspace/useActiveWorkspace";
import { fetchCollectionEntries } from "./queries/fetchCollectionEntries";

export const USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY = "streamCollectionEntries";

export const useStreamedCollectionEntries = (collectionId: string) => {
  const queryClient = useQueryClient();
  const { api } = useTabbedPaneStore();

  const { hasActiveWorkspace } = useActiveWorkspace();

  const query = useQuery<StreamEntriesEvent[], Error>({
    queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collectionId],
    queryFn: async () => {
      const entires = await fetchCollectionEntries(collectionId);

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
    queryClient.resetQueries({ queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY] });
  };

  return {
    ...query,
    clearEntriesCacheAndRefetch,
  };
};
