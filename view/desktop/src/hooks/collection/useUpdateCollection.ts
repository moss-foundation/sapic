import { StreamCollectionsEvent } from "@repo/moss-workspace";
import { useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_COLLECTIONS_QUERY_KEY } from "./useStreamedCollections";

export interface UseUpdateCollectionInput {
  id: string;
  collection: StreamCollectionsEvent;
}

export const useUpdateCollection = () => {
  const queryClient = useQueryClient();

  const placeholderFnForUpdateCollection = ({ id, collection }: UseUpdateCollectionInput) => {
    queryClient.setQueryData([USE_STREAM_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
      return old.map((c) => (c.id === id ? collection : c));
    });
  };

  return {
    placeholderFnForUpdateCollection,
  };
};
