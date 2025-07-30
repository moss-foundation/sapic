import { invokeTauriIpc } from "@/lib/backend/tauri";
import { BatchCreateEntryInput, BatchCreateEntryOutput } from "@repo/moss-collection";
import { useMutation } from "@tanstack/react-query";

interface UseBatchCreateCollectionEntryInput {
  collectionId: string;
  input: BatchCreateEntryInput;
}

const batchCreateCollectionEntry = async ({ collectionId, input }: UseBatchCreateCollectionEntryInput) => {
  const result = await invokeTauriIpc<BatchCreateEntryOutput>("batch_create_collection_entry", {
    collectionId,
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useBatchCreateCollectionEntry = () => {
  return useMutation<BatchCreateEntryOutput, Error, UseBatchCreateCollectionEntryInput>({
    mutationFn: batchCreateCollectionEntry,
  });
};
