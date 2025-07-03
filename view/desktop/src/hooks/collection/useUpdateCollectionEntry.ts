import { invokeTauriIpc } from "@/lib/backend/tauri";
import { EntryInfo, UpdateEntryInput, UpdateEntryOutput } from "@repo/moss-collection";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamedCollectionEntries";

export interface UseUpdateCollectionEntryInput {
  collectionId: string;
  updatedEntry: UpdateEntryInput;
}

const updateCollectionEntry = async ({ collectionId, updatedEntry }: UseUpdateCollectionEntryInput) => {
  const result = await invokeTauriIpc<UpdateEntryOutput>("update_collection_entry", {
    collectionId,
    input: updatedEntry,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateCollectionEntry = (collectionId: string) => {
  const queryClient = useQueryClient();

  return useMutation<UpdateEntryOutput, Error, UseUpdateCollectionEntryInput>({
    mutationFn: updateCollectionEntry,
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collectionId], (old: EntryInfo[]) => {
        return old.map((oldEntry) => {
          const backendEntryData = "ITEM" in data ? data.ITEM : data.DIR;
          const inputEntryData =
            "ITEM" in variables.updatedEntry ? variables.updatedEntry.ITEM : variables.updatedEntry.DIR;

          if (oldEntry.id === backendEntryData.id) {
            return {
              ...oldEntry,
              ...inputEntryData,
              ...backendEntryData,
            };
          }

          return oldEntry;
        });
      });
      queryClient.invalidateQueries({
        queryKey: [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collectionId],
        exact: true,
      });
    },
  });
};
