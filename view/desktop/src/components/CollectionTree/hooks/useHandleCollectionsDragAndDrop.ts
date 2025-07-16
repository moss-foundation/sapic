import { useCallback, useEffect } from "react";

import { useCollectionsTrees } from "@/hooks";
import { useBatchUpdateCollection } from "@/hooks/collection/useBatchUpdateCollection";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeCollectionRootNode } from "../types";

export const useHandleCollectionsDragAndDrop = () => {
  const { collectionsTrees } = useCollectionsTrees();
  const { mutateAsync: batchUpdateCollection } = useBatchUpdateCollection();

  const handleReorder = useCallback(
    async ({ location, source }) => {
      if (!collectionsTrees || location.current?.dropTargets.length === 0) return;

      const sourceData = getTreeRootNodeSourceData(source);
      const targetData = getTreeRootNodeTargetData(location);

      if (targetData.data.collectionId === sourceData.data.collectionId) {
        return;
      }

      try {
        const sortedCollections = [...collectionsTrees].sort((a, b) => (a.order || 0) - (b.order || 0));

        const sourceIndex = sortedCollections.findIndex((collection) => collection.id === sourceData.data.collectionId);
        const targetIndex = sortedCollections.findIndex((collection) => collection.id === targetData.data.collectionId);

        if (sourceIndex === -1 || targetIndex === -1) {
          console.error("Source or target collection not found");
          return;
        }

        const insertionIndex = targetData.data.closestEdge === "top" ? targetIndex : targetIndex + 1;

        const collectionToMove = sortedCollections[sourceIndex];
        const newCollections = [
          ...sortedCollections.slice(0, sourceIndex),
          ...sortedCollections.slice(sourceIndex + 1),
        ];

        const finalCollections = [
          ...newCollections.slice(0, sourceIndex < insertionIndex ? insertionIndex - 1 : insertionIndex),
          collectionToMove,
          ...newCollections.slice(sourceIndex < insertionIndex ? insertionIndex - 1 : insertionIndex),
        ];

        const collectionsWithUpdatedOrder = finalCollections.map((collection, index) => ({
          ...collection,
          order: index + 1,
        }));

        await batchUpdateCollection({
          items: collectionsWithUpdatedOrder.map((collection) => ({
            id: collection.id,
            order: collection.order,
          })),
        });
      } catch (error) {
        console.error("Error reordering collections:", error);
      }
    },
    [collectionsTrees, batchUpdateCollection]
  );

  useEffect(() => {
    return monitorForElements({
      canMonitor: ({ source }) => {
        return source.data.type === "TreeRootNode";
      },
      onDrop: handleReorder,
    });
  }, [handleReorder]);

  const getTreeRootNodeSourceData = (source: ElementDragPayload) => {
    return source.data as {
      type: "TreeRootNode";
      data: {
        collectionId: string;
        node: TreeCollectionRootNode;
      };
    };
  };
  const getTreeRootNodeTargetData = (location: DragLocationHistory) => {
    return {
      type: "TreeRootNode",
      data: location.current?.dropTargets[0].data,
    } as {
      type: "TreeRootNode";
      data: {
        closestEdge: "top" | "bottom";
        collectionId: string;
        node: TreeCollectionRootNode;
      };
    };
  };
};
