import { useCallback, useEffect } from "react";

import { useCollectionsTrees } from "@/hooks";
import { useBatchUpdateCollection } from "@/hooks/collection/useBatchUpdateCollection";
import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeCollectionRootNode } from "../types";

export const useCollectionsDragAndDropHandler = () => {
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
        const sorted = [...collectionsTrees].sort((a, b) => a.order! - b.order!);

        const sourceIndex = sorted.findIndex((collection) => collection.id === sourceData.data.collectionId);
        const targetIndex = sorted.findIndex((collection) => collection.id === targetData.data.collectionId);

        if (sourceIndex === -1 || targetIndex === -1) {
          console.error("Source or target collection not found");
          return;
        }

        const insertAt = targetData.data.instruction.operation === "reorder-before" ? targetIndex : targetIndex + 1;

        const collectionToMove = sorted[sourceIndex];

        const inserted = [
          ...sorted.slice(0, insertAt).filter((collection) => collection.id !== collectionToMove.id),
          collectionToMove,
          ...sorted.slice(insertAt).filter((collection) => collection.id !== collectionToMove.id),
        ];

        const reordered = inserted.map((collection, index) => ({
          ...collection,
          order: index + 1,
        }));

        const collectionsToUpdate = reordered.filter((reorderedCollection) => {
          const collectionUnderQuestion = sorted.find(
            (sortedCollection) => sortedCollection.id === reorderedCollection.id
          );
          return collectionUnderQuestion!.order !== reorderedCollection.order;
        });

        await batchUpdateCollection({
          items: collectionsToUpdate.map((collection) => ({
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
    const instruction = extractInstruction(location.current?.dropTargets[0].data);

    return {
      type: "TreeRootNode",
      data: {
        ...location.current?.dropTargets[0].data,
        instruction,
      },
    } as {
      type: "TreeRootNode";
      data: {
        instruction: Instruction;
        collectionId: string;
        node: TreeCollectionRootNode;
      };
    };
  };
};
