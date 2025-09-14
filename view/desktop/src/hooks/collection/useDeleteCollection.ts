import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { DeleteCollectionInput, DeleteCollectionOutput, StreamCollectionsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useStreamedCollectionsWithEntries } from "..";
import { USE_STREAM_COLLECTIONS_QUERY_KEY } from "./useStreamCollections";

export interface UseDeleteCollectionInput {
  id: string;
}

const deleteStreamedCollection = async ({ id }: DeleteCollectionInput) => {
  const result = await invokeTauriIpc<DeleteCollectionOutput>("delete_project", { input: { id } });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteCollection = () => {
  const queryClient = useQueryClient();
  const { api } = useTabbedPaneStore();
  const { data: collectionsWithEntries } = useStreamedCollectionsWithEntries();

  return useMutation({
    mutationFn: deleteStreamedCollection,
    onSuccess: (data) => {
      queryClient.setQueryData([USE_STREAM_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
        return old.filter((collection) => collection.id !== data.id);
      });

      collectionsWithEntries?.forEach((collection) => {
        if (collection.id === data.id) {
          const collectionPanel = api?.getPanel(collection.id);

          if (collectionPanel) {
            api?.removePanel(collectionPanel);
          }

          collection.entries.forEach((entry) => {
            const panel = api?.getPanel(entry.id);
            if (panel) {
              api?.removePanel(panel);
            }
          });
        }
      });
    },
  });
};
