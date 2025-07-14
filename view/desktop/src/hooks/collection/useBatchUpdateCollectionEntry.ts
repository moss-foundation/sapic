import { invokeTauriIpc } from "@/lib/backend/tauri";
import { BatchUpdateEntryInput, BatchUpdateEntryOutput, BatchUpdateEntryOutputKind } from "@repo/moss-collection";
import { useMutation } from "@tanstack/react-query";
import { Channel } from "@tauri-apps/api/core";

import { useFetchEntriesForPath } from "./derivedHooks/useFetchEntriesForPath";

export interface UseBatchUpdateCollectionEntryInput {
  collectionId: string;
  entries: BatchUpdateEntryInput;
}

const batchUpdateCollectionEntry = async ({ collectionId, entries }: UseBatchUpdateCollectionEntryInput) => {
  const onCollectionEvent = new Channel<BatchUpdateEntryOutputKind>();

  await invokeTauriIpc<BatchUpdateEntryOutput>("batch_update_collection_entry", {
    channel: onCollectionEvent,
    collectionId,
    input: {
      entries: entries.entries,
    },
  });
};

export const useBatchUpdateCollectionEntry = () => {
  const { fetchEntriesForPath } = useFetchEntriesForPath();

  //TODO output type is ambiguous and a little bit hardcoded
  return useMutation<void, Error, UseBatchUpdateCollectionEntryInput>({
    mutationFn: batchUpdateCollectionEntry,
    onSuccess: async (data, mutationInput) => {
      // console.log({
      //   data,
      //   mutationInput,
      // });
      // if (mutationInput.entries.entries.length > 0) {
      //   const firstEntry = mutationInput.entries.entries[0];
      //   if ("DIR" in firstEntry) {
      //     const res = await fetchCollectionEntries(mutationInput.collectionId, firstEntry.DIR.path);
      //     console.log({
      //       res,
      //     });
      //   }
      //   if ("ITEM" in firstEntry) {
      //     const res = await fetchCollectionEntries(mutationInput.collectionId, firstEntry.ITEM.path);
      //     console.log({
      //       res,
      //     });
      //   }
      // }
      // const normalizedUpdatedEntries = updatedEntriesFromServer.map((entry) => {
      //   if ("ITEM" in entry) return entry.ITEM;
      //   if ("DIR" in entry) return entry.DIR;
      //   return entry;
      // });
      // const normalizedInputEntries = mutationInput.entries.entries.map((entry) => {
      //   if ("ITEM" in entry) return entry.ITEM;
      //   if ("DIR" in entry) return entry.DIR;
      //   return entry;
      // });
      // queryClient.setQueryData(
      //   [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, mutationInput.collectionId],
      //   (currentEntries: EntryInfo[]) => {
      //     return currentEntries.map((existingEntry) => {
      //       const serverEntry = normalizedUpdatedEntries.find((entry) => existingEntry.id === entry.id);
      //       const inputEntry = normalizedInputEntries.find((entry) => existingEntry.id === entry.id);
      //       if (serverEntry) {
      //         return {
      //           ...existingEntry,
      //           ...inputEntry,
      //           ...serverEntry,
      //         };
      //       }
      //       return existingEntry;
      //     });
      //   }
      // );
    },
  });
};

const getNormalizedEntries = (entries: BatchUpdateEntryOutputKind[]) => {
  return entries.map((entry) => {
    if ("ITEM" in entry) return entry.ITEM;
    if ("DIR" in entry) return entry.DIR;
    return entry;
  });
};
