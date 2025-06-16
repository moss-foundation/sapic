import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DeleteCollectionInput, DeleteCollectionOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { USE_LIST_COLLECTIONS_QUERY_KEY } from "./useListCollections";

export const USE_DELETE_COLLECTION_MUTATION_KEY = "deleteCollection";

const deleteCollectionFn = async (input: DeleteCollectionInput): Promise<DeleteCollectionOutput> => {
  const result = await invokeTauriIpc<DeleteCollectionOutput>("delete_collection", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteCollection = () => {
  const queryClient = useQueryClient();
  return useMutation<DeleteCollectionOutput, Error, DeleteCollectionInput>({
    mutationKey: [USE_DELETE_COLLECTION_MUTATION_KEY],
    mutationFn: deleteCollectionFn,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_LIST_COLLECTIONS_QUERY_KEY] });
    },
  });
};
