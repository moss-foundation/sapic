import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CreateCollectionInput, CreateCollectionOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "./useStreamedCollections";

const createCollection = async (collection: CreateCollectionInput) => {
  const result = await invokeTauriIpc<CreateCollectionOutput>("create_collection", { input: collection });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateCollection = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (collection: CreateCollectionInput) => createCollection(collection),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY] });
    },
  });
};
