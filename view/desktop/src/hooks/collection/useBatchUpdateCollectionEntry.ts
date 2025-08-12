import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { BatchUpdateEntryInput, BatchUpdateEntryOutput, BatchUpdateEntryOutputKind } from "@repo/moss-collection";
import { useMutation } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

export interface UseBatchUpdateCollectionEntryInput {
  collectionId: string;
  entries: BatchUpdateEntryInput;
}

const batchUpdateCollectionEntry = async ({ collectionId, entries }: UseBatchUpdateCollectionEntryInput) => {
  console.log("batchUpdateCollectionEntry", entries);
  const onCollectionEvent = new Channel<BatchUpdateEntryOutputKind>();

  const result = await invokeTauriIpc<BatchUpdateEntryOutput>("batch_update_collection_entry", {
    channel: onCollectionEvent,
    collectionId,
    input: {
      entries: entries.entries,
    },
  });

  return result;
};

export const useBatchUpdateCollectionEntry = () => {
  return useMutation<IpcResult<BatchUpdateEntryOutput, unknown>, Error, UseBatchUpdateCollectionEntryInput>({
    mutationFn: batchUpdateCollectionEntry,
  });
};
