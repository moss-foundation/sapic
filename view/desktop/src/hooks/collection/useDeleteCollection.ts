import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useCollectionsStore } from "@/store/collections";

interface DeleteCollectionParams {
  id: string;
}

export const useDeleteCollection = () => {
  const queryClient = useQueryClient();
  const { deleteCollection: deleteFromStore } = useCollectionsStore();

  return useMutation({
    mutationFn: async ({ id }: DeleteCollectionParams) => {
      await deleteFromStore(id);
      return { id };
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["listCollections"] });
    },
  });
};
