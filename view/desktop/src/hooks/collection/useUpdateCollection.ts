import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invokeTauriIpc } from "@/lib/backend/tauri";
import { UpdateCollectionInput, UpdateCollectionOutput } from "@repo/moss-workspace";
import { useCollectionsStore } from "@/store/collections";

interface UpdateCollectionParams {
  collectionId: string;
  name?: string;
  order?: number;
  pinned?: boolean;
}

export const useUpdateCollection = () => {
  const queryClient = useQueryClient();
  const { updateCollection: updateStoreCollection } = useCollectionsStore();

  return useMutation({
    mutationFn: async ({ collectionId, name, order, pinned }: UpdateCollectionParams) => {
      const input: UpdateCollectionInput = {
        id: collectionId,
        newName: name || null,
        order,
        pinned,
      };

      const result = await invokeTauriIpc<UpdateCollectionOutput>("update_collection", { input });

      if (result.status === "error") {
        throw new Error(String(result.error));
      }

      return result.data;
    },
    onSuccess: (data, variables) => {
      // Update the store if we have the collection data
      const collections = useCollectionsStore.getState().collections;
      const targetCollection = collections.find(
        (c) => (typeof c.id === "string" ? c.id : String(c.id)) === variables.collectionId
      );

      if (targetCollection && variables.name) {
        const updatedCollection = {
          ...targetCollection,
          id: variables.name, // Update the display name in the store
        };
        updateStoreCollection(updatedCollection);
      }

      // Invalidate related queries
      queryClient.invalidateQueries({ queryKey: ["listCollections"] });
    },
  });
};
