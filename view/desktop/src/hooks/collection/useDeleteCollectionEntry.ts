import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DeleteEntryInput, DeleteEntryOutput, EntryInfo } from "@repo/moss-collection";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamedCollectionEntries";

export interface UseDeleteCollectionEntryInput {
  collectionId: string;
  input: DeleteEntryInput;
}

const deleteCollectionEntry = async ({ collectionId, input }: UseDeleteCollectionEntryInput) => {
  const result = await invokeTauriIpc<DeleteEntryOutput>("delete_collection_entry", { collectionId, input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteCollectionEntry = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteCollectionEntry,
    onSuccess: (data, variables) => {
      queryClient.setQueryData(
        [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, variables.collectionId],
        (old: EntryInfo[]) => {
          return old.filter((entry) => entry.id !== data.id);
        }
      );
    },
  });
};
