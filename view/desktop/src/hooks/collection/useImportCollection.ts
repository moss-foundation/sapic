import { invokeTauriIpc } from "@/lib/backend/tauri";
import { ImportCollectionInput, ImportCollectionOutput, StreamCollectionsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "./useStreamedCollections";

export const IMPORT_COLLECTION_QUERY_KEY = "importCollection";

const importCollection = async (input: ImportCollectionInput) => {
  const result = await invokeTauriIpc<ImportCollectionOutput>("import_collection", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useImportCollection = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: [IMPORT_COLLECTION_QUERY_KEY],
    mutationFn: importCollection,
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_STREAMED_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
        return [
          ...old,
          {
            ...data,
            ...variables,
          },
        ];
      });
    },
  });
};
