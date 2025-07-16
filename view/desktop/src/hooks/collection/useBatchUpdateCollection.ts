import { StreamCollectionsEvent } from "@repo/moss-workspace";
import { useQueryClient } from "@tanstack/react-query";

import { USE_STREAMED_COLLECTIONS_QUERY_KEY } from "./useStreamedCollections";

export interface UseUpdateCollectionInput {
  id: string;
  collection: StreamCollectionsEvent;
}

export const useBatchUpdateCollection = () => {
  const queryClient = useQueryClient();

  const placeholderFnForBatchUpdateCollection = ({ collections }: { collections: UseUpdateCollectionInput[] }) => {
    queryClient.setQueryData([USE_STREAMED_COLLECTIONS_QUERY_KEY], (old: StreamCollectionsEvent[]) => {
      return old.map((oldCollection) => {
        const updatedCollection = collections.find((collection) => collection.id === oldCollection.id);
        if (updatedCollection) {
          return updatedCollection.collection;
        }

        return oldCollection;
      });
    });
  };

  return {
    placeholderFnForBatchUpdateCollection,
  };
};
