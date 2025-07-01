import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DeleteCollectionOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "./useStreamedCollections";

export interface UseDeleteCollectionInput {
  id: string;
}

const deleteStreamedCollection = async ({ id }: UseDeleteCollectionInput) => {
  const result = await invokeTauriIpc<DeleteCollectionOutput>("delete_collection", { input: { id } });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteCollection = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteStreamedCollection,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [USE_STREAMED_COLLECTIONS_QUERY_KEY] });
    },
  });
};
