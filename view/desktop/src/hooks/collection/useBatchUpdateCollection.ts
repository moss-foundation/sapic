import { invokeTauriIpc } from "@/lib/backend/tauri";
import { BatchUpdateCollectionInput, StreamCollectionsEvent, UpdateCollectionOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_COLLECTIONS_QUERY_KEY } from "./useStreamCollections";

const batchUpdateCollection = async (input: BatchUpdateCollectionInput) => {
  const result = await invokeTauriIpc<UpdateCollectionOutput>("batch_update_collection", {
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useBatchUpdateCollection = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: batchUpdateCollection,
    onSuccess: (_, variables) => {
      queryClient.setQueryData([USE_STREAM_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
        return old.map((oldCollection) => {
          const updatedCollection = variables.items.find((collection) => collection.id === oldCollection.id);
          if (updatedCollection) {
            return {
              ...oldCollection,
              ...updatedCollection,
            };
          }

          return oldCollection;
        });
      });
    },
  });
};
