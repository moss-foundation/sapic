import { USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, useDeleteCollectionEntry } from "@/hooks";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { StreamEntriesEvent } from "@repo/moss-collection";
import { useQueryClient } from "@tanstack/react-query";

import { ProjectTreeNode, ProjectTreeRootNode } from "../types";
import { siblingsAfterRemovalPayload } from "../utils";

export const useDeleteAndUpdatePeers = (
  collectionId: string,
  node: ProjectTreeNode,
  parentNode: ProjectTreeNode | ProjectTreeRootNode
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

    const sortedChildren = sortObjectsByOrder(parentNode.childNodes);
    const index = sortedChildren.findIndex((e) => e.id === node.id) + 1;
    const updatedParentNodeChildren = sortedChildren.slice(index).map((e) => ({
      ...e,
      order: e.order! - 1,
    }));

    const result = await batchUpdateCollectionEntry({
      collectionId,
      entries: {
        entries: siblingsAfterRemovalPayload({
          nodes: parentNode.childNodes,
          removedNode: node,
        }),
      },
    });

    if (result.status === "ok") {
      queryClient.setQueryData(
        [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, collectionId],
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
