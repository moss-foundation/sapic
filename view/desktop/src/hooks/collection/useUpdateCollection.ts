import { invokeTauriIpc } from "@/lib/backend/tauri";
import { UpdateEntryOutput } from "@repo/moss-collection";
import { StreamCollectionsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "./useStreamedCollections";

export interface UseUpdateCollectionInput {
  id: string;
  collection: StreamCollectionsEvent;
}
export const updateCollection = async ({ id, collection }: UseUpdateCollectionInput) => {
  const result = await invokeTauriIpc<UpdateEntryOutput>("update_collection", {
    id,
    input: { collection },
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateCollection = () => {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: updateCollection,
    onSuccess: (_, variables) => {
      queryClient.setQueryData([USE_STREAMED_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
        return old.map((c) => (c.id === variables.id ? variables.collection : c));
      });
    },
  });

  const placeholderFnForUpdateCollection = ({ id, collection }: UseUpdateCollectionInput) => {
    queryClient.setQueryData([USE_STREAMED_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
      return old.map((c) => (c.id === id ? collection : c));
    });
  };

  return {
    ...mutation,
    placeholderFnForUpdateCollection,
  };
};
