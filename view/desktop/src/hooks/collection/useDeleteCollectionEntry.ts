import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DeleteEntryInput, DeleteEntryOutput } from "@repo/moss-collection";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamCollectionEntries";

export interface UseDeleteCollectionEntryInput {
  collectionId: string;
  input: DeleteEntryInput;
}

const deleteCollectionEntry = async ({ collectionId, input }: UseDeleteCollectionEntryInput) => {
  const result = await invokeTauriIpc<DeleteEntryOutput>("delete_collection_entry", { collectionId, input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }
};

export const useDeleteCollectionEntry = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteCollectionEntry,
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, variables.collectionId] });
    },
  });
};
