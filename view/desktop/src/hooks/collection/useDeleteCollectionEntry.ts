import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DeleteEntryInput, DeleteEntryOutput } from "@repo/moss-collection";
import { useMutation } from "@tanstack/react-query";

export const USE_DELETE_COLLECTION_ENTRY_MUTATION_KEY = "deleteCollectionEntry" as const;

const deleteCollectionEntryFn = async ({ collectionId, input }: { collectionId: string; input: DeleteEntryInput }) => {
  const result = await invokeTauriIpc<DeleteEntryOutput>("delete_collection_entry", {
    collectionId,
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteCollectionEntry = () => {
  return useMutation<DeleteEntryOutput, Error, { collectionId: string; input: DeleteEntryInput }>({
    mutationKey: [USE_DELETE_COLLECTION_ENTRY_MUTATION_KEY],
    mutationFn: deleteCollectionEntryFn,
  });
};
