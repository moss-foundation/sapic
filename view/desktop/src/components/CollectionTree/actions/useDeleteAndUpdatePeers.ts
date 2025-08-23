import { USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, useDeleteCollectionEntry } from "@/hooks";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { BatchUpdateEntryInput, BatchUpdateEntryKind, StreamEntriesEvent } from "@repo/moss-collection";
import { useQueryClient } from "@tanstack/react-query";

import { TreeCollectionNode } from "../types";
import { sortByOrder } from "../utils";

export const useDeleteAndUpdatePeers = (
  collectionId: string,
  node: TreeCollectionNode,
  parentNode: TreeCollectionNode
) => {
  const queryClient = useQueryClient();

  const { mutateAsync: deleteCollectionEntry } = useDeleteCollectionEntry();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();

  const deleteAndUpdatePeers = async () => {
    await deleteCollectionEntry({
      collectionId,
      input: {
        id: node.id,
      },
    });

    const sortedChildren = sortByOrder(parentNode.childNodes);
    const index = sortedChildren.findIndex((e) => e.id === node.id) + 1;
    const updatedParentNodeChildren = sortedChildren.slice(index).map((e) => ({
      ...e,
      order: e.order! - 1,
    }));

    const input: BatchUpdateEntryInput = {
      entries: updatedParentNodeChildren.map((e): BatchUpdateEntryKind => {
        if (e.kind === "Dir") {
          return {
            DIR: {
              id: e.id,
              order: e.order,
            },
          };
        }

        return {
          ITEM: {
            id: e.id,
            order: e.order,
            queryParamsToAdd: [],
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: [],
            pathParamsToRemove: [],
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
          },
        };
      }),
    };

    const result = await batchUpdateCollectionEntry({
      collectionId,
      entries: input,
    });

    if (result.status === "ok") {
      queryClient.setQueryData(
        [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collectionId],
        (cacheData: StreamEntriesEvent[]) => {
          return cacheData.map((cacheEntry) => {
            if (updatedParentNodeChildren.some((e) => e.id === cacheEntry.id)) {
              const updatedEntry = updatedParentNodeChildren.find((e) => e.id === cacheEntry.id);
              return { ...cacheEntry, ...updatedEntry };
            }

            return cacheEntry;
          });
        }
      );
    }
  };

  return {
    deleteAndUpdatePeers,
  };
};
