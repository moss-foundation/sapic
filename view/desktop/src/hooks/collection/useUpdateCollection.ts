// import { invokeTauriIpc } from "@/lib/backend/tauri";
// import { UpdateCollectionInput, UpdateCollectionOutput } from "@repo/moss-workspace";

interface UpdateCollectionParams {
  collectionId: string;
  name?: string;
  order?: number;
  pinned?: boolean;
}

export const useUpdateCollection = () => {
  // const queryClient = useQueryClient();
  // const { updateCollection: updateStoreCollection } = useCollectionsStore();
  // return useMutation({
  //   mutationFn: async ({ collectionId, name, order, pinned }: UpdateCollectionParams) => {
  //     // Mock implementation - simulate network delay
  //     await new Promise((resolve) => setTimeout(resolve, 200));
  // TODO: Replace with actual backend call when update_collection is implemented
  // const input: UpdateCollectionInput = {
  //   id: collectionId,
  //   newName: name || undefined,
  //   order,
  //   pinned,
  // };
  //
  // const result = await invokeTauriIpc<UpdateCollectionOutput>("update_collection", { input });
  //
  // if (result.status === "error") {
  //   throw new Error(String(result.error));
  // }
  //
  // return result.data;
  // Mock successful response
  //     return {
  //       collection: {
  //         id: collectionId,
  //         name: name || `Collection ${collectionId}`,
  //         order: order || 0,
  //         pinned: pinned || false,
  //       },
  //     };
  //   },
  //   onSuccess: (_data, variables) => {
  //     // Update the store if we have the collection data
  //     const collections = useCollectionsStore.getState().collections;
  //     const targetCollection = collections.find(
  //       (c) => (typeof c.id === "string" ? c.id : String(c.id)) === variables.collectionId
  //     );
  //     if (targetCollection && variables.name) {
  //       const updatedCollection = {
  //         ...targetCollection,
  //         id: variables.name, // Update the display name in the store
  //       };
  //       updateStoreCollection(updatedCollection);
  //     }
  //     // Invalidate related queries
  //     queryClient.invalidateQueries({ queryKey: ["listCollections"] });
  //   },
  // });
};
