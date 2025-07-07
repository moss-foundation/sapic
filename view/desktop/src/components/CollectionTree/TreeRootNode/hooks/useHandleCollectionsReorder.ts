import { useEffect } from "react";

import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { ElementDragPayload, monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeCollectionRootNode } from "../../types";

export const useHandleCollectionsReorder = () => {
  useEffect(() => {
    monitorForElements({
      canMonitor: ({ source }) => {
        return source.data.type === "TreeRootNode";
      },
      onDrop({ location, source }) {
        if (location.current?.dropTargets.length === 0) return;

        const sourceData = getTreeRootNodeSource(source);
        const targetData = getTreeRootNodeTarget(location);
      },
    });
  }, []);
};

const getTreeRootNodeSource = (source: ElementDragPayload) => {
  return source.data as {
    type: "TreeRootNode";
    data: {
      treeId: string;
      node: TreeCollectionRootNode;
    };
  };
};

const getTreeRootNodeTarget = (location: DragLocationHistory) => {
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
