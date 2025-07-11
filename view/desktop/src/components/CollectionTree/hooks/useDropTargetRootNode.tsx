import { RefObject, useContext, useEffect, useState } from "react";

import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeContext } from "../Tree";
import { TreeCollectionRootNode } from "../types";
import { getActualDropSourceTarget, getActualDropTarget } from "../utils";
import { canDropRootNode } from "../utils2";

export const useDropTargetRootNode = (node: TreeCollectionRootNode, dropTargetListRef: RefObject<HTMLDivElement>) => {
  const { id } = useContext(TreeContext);

  const [isDragging, setIsDragging] = useState(false);
  const [canDrop, setCanDrop] = useState(false);

  useEffect(() => {
    const element = dropTargetListRef.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: () => {
        return { type: "TreeRootNode", data: { collectionId: id, node } };
      },
      canDrop({ source }) {
        return source.data.type === "TreeNode" || source.data.type === "TreeRootNode";
      },
      onDropTargetChange({ location, source }) {
        if (
          location.current?.dropTargets.length === 0 ||
          location.current.dropTargets[0].data.type !== "TreeRootNode"
        ) {
          setCanDrop(false);
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (!dropTarget || !sourceTarget || dropTarget?.node.id !== node.id) {
          setCanDrop(false);
          return;
        }

        setCanDrop(canDropRootNode(sourceTarget, dropTarget, node));
      },
      onDrop() {
        setCanDrop(false);
        setIsDragging(false);
      },
    });
  }, [dropTargetListRef, node, id]);

  return { canDropRootNode: canDrop, isDraggingRootNode: isDragging };
};
