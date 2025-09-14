import { invokeTauriIpc } from "@/lib/backend/tauri";
import { CreateEntryInput, CreateEntryOutput, StreamEntriesEvent } from "@repo/moss-collection";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY } from "./useStreamCollectionEntries";
import { createCollectionEntryForCache } from "./utils";

export interface UseCreateCollectionEntryInputProps {
  collectionId: string;
  input: CreateEntryInput;
}

const createCollectionEntry = async ({ collectionId, input }: UseCreateCollectionEntryInputProps) => {
  console.log("createCollectionEntry", collectionId, input);
  const result = await invokeTauriIpc<CreateEntryOutput>("create_collection_entry", {
    collectionId,
    input,
  });

  console.log("result", result);

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateCollectionEntry = () => {
  const queryClient = useQueryClient();

  return useMutation<CreateEntryOutput, Error, UseCreateCollectionEntryInputProps>({
    mutationFn: createCollectionEntry,
    onSuccess: async (data, variables) => {
      const newEntry = await createCollectionEntryForCache(data.id, variables.input);

      queryClient.setQueryData(
        [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, variables.collectionId],
        (old: StreamEntriesEvent[]) => {
          return [...old, newEntry];
        }
      );
    },
  });
};
