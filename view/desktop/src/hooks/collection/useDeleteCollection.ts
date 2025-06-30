import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useCollectionsStore } from "@/store/collections";
import { DeleteCollectionOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "./useStreamedCollections";

const deleteStreamedCollection = async (id: string) => {
  const result = await invokeTauriIpc<DeleteCollectionOutput>("delete_collection", { input: { id } });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteCollection = () => {
  const queryClient = useQueryClient();
  const { deleteCollection: deleteFromStore } = useCollectionsStore();

  return useMutation({
    mutationFn: async (id: string) => {
      await deleteFromStore(id); //TODO: remove this once the collections store is removed
      await deleteStreamedCollection(id);

      return id;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY] });
    },
  });
};
