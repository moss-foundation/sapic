import { invokeTauriIpc } from "@/lib/backend/tauri";
import { BatchUpdateEntryInput, BatchUpdateEntryOutput, EntryInfo } from "@repo/moss-collection";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamedCollectionEntries";

interface UseBatchUpdateCollectionEntry {
  collectionId: string;
  entries: BatchUpdateEntryInput;
}

const batchUpdateCollectionEntry = async ({ collectionId, entries }: UseBatchUpdateCollectionEntry) => {
  const batch: BatchUpdateEntryOutput[] = [];

  const onBatchUpdateCollectionEntryEvent = new Channel<BatchUpdateEntryOutput>();

  onBatchUpdateCollectionEntryEvent.onmessage = (Event) => {
    batch.push(Event);
  };

  await invokeTauriIpc("batch_update_collection_entry", {
    channel: onBatchUpdateCollectionEntryEvent,
    collectionId,
    input: entries,
  });

  return batch;
};

export const useBatchUpdateCollectionEntry = () => {
  const queryClient = useQueryClient();
  return useMutation<BatchUpdateEntryOutput[], Error, UseBatchUpdateCollectionEntry>({
    mutationFn: batchUpdateCollectionEntry,
    onSuccess: (data, variables) => {
      const normalizedUpdatedEntries = data.map((data) => {
        if ("ITEM" in data) return data.ITEM;
        if ("DIR" in data) return data.DIR;

        return data;
      });

      const normalizedInputEntries = variables.entries.entries.map((entry) => {
        if ("ITEM" in entry) return entry.ITEM;
        if ("DIR" in entry) return entry.DIR;

        return entry;
      });

      queryClient.setQueryData(
        [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, variables.collectionId],
        (old: EntryInfo[]) => {
          const newEntries = old.map((oldEntry) => {
            const backendEntryData = normalizedUpdatedEntries.find((data) => oldEntry.id === data.id);
            const inputEntry = normalizedInputEntries.find((data) => oldEntry.id === data.id);

            if (backendEntryData) {
              return {
                ...oldEntry,
                ...inputEntry,
                ...backendEntryData,
              };
            }

            return oldEntry;
          });

          return newEntries;
        }
      );
    },
  });
};
