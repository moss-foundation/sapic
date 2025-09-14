import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { BatchUpdateEntryInput, BatchUpdateEntryOutput, BatchUpdateEntryOutputKind } from "@repo/moss-project";
import { useMutation } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

export interface UseBatchUpdateCollectionEntryInput {
  collectionId: string;
  entries: BatchUpdateEntryInput;
}

const batchUpdateCollectionEntry = async ({ collectionId, entries }: UseBatchUpdateCollectionEntryInput) => {
  const onCollectionEvent = new Channel<BatchUpdateEntryOutputKind>();

  const result = await invokeTauriIpc<BatchUpdateEntryOutput>("batch_update_project_entry", {
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
