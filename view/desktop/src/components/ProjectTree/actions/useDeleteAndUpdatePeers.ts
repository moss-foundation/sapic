import { USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, useDeleteProjectEntry } from "@/hooks";
import { useBatchUpdateProjectEntry } from "@/hooks/project/useBatchUpdateProjectEntry";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { StreamEntriesEvent } from "@repo/moss-project";
import { useQueryClient } from "@tanstack/react-query";

import { ProjectTreeNode, ProjectTreeRootNode } from "../types";
import { siblingsAfterRemovalPayload } from "../utils";

export const useDeleteAndUpdatePeers = (
  collectionId: string,
  node: ProjectTreeNode,
  parentNode: ProjectTreeNode | ProjectTreeRootNode
) => {
  const queryClient = useQueryClient();

  const { mutateAsync: deleteCollectionEntry } = useDeleteProjectEntry();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateProjectEntry();

  const deleteAndUpdatePeers = async () => {
    await deleteCollectionEntry({
      projectId: collectionId,
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
      projectId: collectionId,
      entries: {
        entries: siblingsAfterRemovalPayload({
          nodes: parentNode.childNodes,
          removedNode: node,
        }),
      },
    });

    if (result.status === "ok") {
      queryClient.setQueryData(
        [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, collectionId],
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
