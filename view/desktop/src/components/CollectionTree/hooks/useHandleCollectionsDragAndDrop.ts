import { useCallback, useEffect } from "react";

import { useCollectionsStore } from "@/store/collections";
import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { ElementDragPayload, monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { swapListById } from "../../../utils/swapListById";
import { TreeCollectionRootNode } from "../types";

export const useHandleCollectionsDragAndDrop = () => {
  const { collectionsTrees, setCollectionsTrees } = useCollectionsStore();
  const handleReorder = useCallback(
    ({ location, source }) => {
      if (location.current?.dropTargets.length === 0) return;

      const sourceData = getTreeRootNodeSourceData(source);
      const targetData = getTreeRootNodeTargetData(location);

      if (targetData.data.treeId === sourceData.data.treeId) {
        return;
      }

      const reorderedTrees = swapListById(
        sourceData.data.treeId,
        targetData.data.treeId,
        collectionsTrees,
        targetData.data.closestEdge
      );
      if (reorderedTrees) {
        setCollectionsTrees(reorderedTrees);
      }
    },
    [collectionsTrees, setCollectionsTrees]
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
        treeId: string;
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
        treeId: string;
        node: TreeCollectionRootNode;
      };
    };
  };
};
