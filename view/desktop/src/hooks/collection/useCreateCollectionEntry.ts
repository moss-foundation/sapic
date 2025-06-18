import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CreateEntryInput, CreateEntryOutput } from "@repo/moss-collection";
import { useMutation } from "@tanstack/react-query";

export const USE_CREATE_COLLECTION_ENTRY_MUTATION_KEY = "createCollectionEntry" as const;

const createCollectionEntryFn = async ({ collectionId, input }: { collectionId: string; input: CreateEntryInput }) => {
  const result = await invokeTauriIpc<CreateEntryOutput>("create_collection_entry", {
    collectionId,
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateCollectionEntry = () => {
  return useMutation<CreateEntryOutput, Error, { collectionId: string; input: CreateEntryInput }>({
    mutationKey: [USE_CREATE_COLLECTION_ENTRY_MUTATION_KEY],
    mutationFn: createCollectionEntryFn,
  });
};
