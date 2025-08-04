import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";
import { getLocationTreeNodeData, getSourceTreeNodeData, hasDirectSimilarDescendant } from "../utils";

interface UseDropTargetDirNodeProps {
  node: TreeCollectionNode;
  dropTargetRootRef: RefObject<HTMLDivElement>;
}

export const useDropTargetRootDirNode = ({ node, dropTargetRootRef }: UseDropTargetDirNodeProps) => {
  const { id } = useContext(TreeContext);

  const [isChildDropBlocked, setIsChildDropBlocked] = useState<boolean | null>(null);

  useEffect(() => {
    const element = dropTargetRootRef.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: ({ input, element }) => {
        const data = {
          type: "TreeRootNode",
          data: {
            collectionId: id,
            node,
          },
        };

        return attachInstruction(data, {
          input,
          element,
          operations: {
            "reorder-before": "not-available",
            "reorder-after": "not-available",
            combine: "available",
          },
        });
      },
      canDrop({ source }) {
        return source.data.type === "TreeNode";
      },
      onDropTargetChange({ location, source }) {
        const sourceTarget = getSourceTreeNodeData(source);
        const dropTarget = getLocationTreeNodeData(location);

        if (!sourceTarget || !dropTarget) {
          setIsChildDropBlocked(null);
          return;
        }

        if (dropTarget.node.id === node.id) {
          setIsChildDropBlocked(hasDirectSimilarDescendant(node, sourceTarget.node));
        } else {
          setIsChildDropBlocked(null);
        }
      },
      onDrop() {
        setIsChildDropBlocked(null);
      },
    });
  }, [id, node, dropTargetRootRef]);

  return { isChildDropBlocked };
};
