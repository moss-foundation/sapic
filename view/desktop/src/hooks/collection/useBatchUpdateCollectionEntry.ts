import { invokeTauriIpc } from "@/lib/backend/tauri";
import { BatchUpdateEntryInput, BatchUpdateEntryOutput, EntryInfo } from "@repo/moss-collection";
import { StreamCollectionsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamedCollectionEntries";

export interface UseBatchUpdateCollectionEntryInput {
  collectionId: string;
  entries: BatchUpdateEntryInput;
}

const batchUpdateCollectionEntry = async ({ collectionId, entries }: UseBatchUpdateCollectionEntryInput) => {
  const onCollectionEvent = new Channel<StreamCollectionsEvent>();

  onCollectionEvent.onmessage = (collection) => {
    console.log(collection);
  };

  const result = await invokeTauriIpc<BatchUpdateEntryOutput[]>("batch_update_collection_entry", {
    channel: onCollectionEvent,
    collectionId,
    input: {
      entries: entries.entries,
    },
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useBatchUpdateCollectionEntry = () => {
  const queryClient = useQueryClient();

  return useMutation<BatchUpdateEntryOutput[], Error, UseBatchUpdateCollectionEntryInput>({
    mutationFn: batchUpdateCollectionEntry,
    onSuccess: (updatedEntriesFromServer, mutationInput) => {
      const normalizedUpdatedEntries = updatedEntriesFromServer.map((entry) => {
        if ("ITEM" in entry) return entry.ITEM;
        if ("DIR" in entry) return entry.DIR;
        return entry;
      });

      const normalizedInputEntries = mutationInput.entries.entries.map((entry) => {
        if ("ITEM" in entry) return entry.ITEM;
        if ("DIR" in entry) return entry.DIR;
        return entry;
      });

      queryClient.setQueryData(
        [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, mutationInput.collectionId],
        (currentEntries: EntryInfo[]) => {
          return currentEntries.map((existingEntry) => {
            const serverEntry = normalizedUpdatedEntries.find((entry) => existingEntry.id === entry.id);

            const inputEntry = normalizedInputEntries.find((entry) => existingEntry.id === entry.id);

            if (serverEntry) {
              return {
                ...existingEntry,
                ...inputEntry,
                ...serverEntry,
              };
            }

            return existingEntry;
          });
        }
      );
    },
  });
};
