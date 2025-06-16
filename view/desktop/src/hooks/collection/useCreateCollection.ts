import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CreateCollectionInput, CreateCollectionOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { USE_LIST_COLLECTIONS_QUERY_KEY } from "./useListCollections";

export const USE_CREATE_COLLECTION_MUTATION_KEY = "createCollection";

const createCollectionFn = async (input: CreateCollectionInput): Promise<CreateCollectionOutput> => {
  const result = await invokeTauriIpc<CreateCollectionOutput>("create_collection", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateCollection = () => {
  const queryClient = useQueryClient();
  return useMutation<CreateCollectionOutput, Error, CreateCollectionInput>({
    mutationKey: [USE_CREATE_COLLECTION_MUTATION_KEY],
    mutationFn: createCollectionFn,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_LIST_COLLECTIONS_QUERY_KEY] });
    },
  });
};
