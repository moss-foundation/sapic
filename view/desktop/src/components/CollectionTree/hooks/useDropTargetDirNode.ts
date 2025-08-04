import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";
import { getLocationTreeNodeData, getSourceTreeNodeData, hasDirectSimilarDescendant } from "../utils";

interface UseDropTargetDirNodeProps {
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
  dropTargetListRef: RefObject<HTMLLIElement>;
  isLastChild: boolean;
  isRootNode: boolean;
}

export const useDropTargetDirNode = ({
  node,
  parentNode,
  dropTargetListRef,
  isLastChild,
  isRootNode,
}: UseDropTargetDirNodeProps) => {
  const { id } = useContext(TreeContext);

  const [isChildDropBlocked, setIsChildDropBlocked] = useState<boolean | null>(null);

  useEffect(() => {
    const element = dropTargetListRef.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: ({ input, element }) => {
        const data = {
          type: "TreeNode",
          data: {
            collectionId: id,
            node,
            parentNode,
          },
        };

        return attachInstruction(data, {
          input,
          element,
          operations: {
            "reorder-before": isRootNode ? "not-available" : "available",
            "reorder-after": isRootNode || (node.kind === "Dir" && node.expanded) ? "not-available" : "available",
            combine: node.kind === "Dir" ? "available" : "not-available",
          },
        });
      },
      canDrop({ source }) {
        return source.data.type === "TreeNode";
      },
      onDropTargetChange({ location, source, self }) {
        const sourceTarget = getSourceTreeNodeData(source);
        const dropTarget = getLocationTreeNodeData(location);

        if (!sourceTarget || !dropTarget) {
          setIsChildDropBlocked(null);
          return;
        }

        const instruction: Instruction | null = extractInstruction(self.data);

        if (dropTarget.node.id === node.id || dropTarget.parentNode.id === node.id) {
          if (instruction?.operation !== "combine") {
            setIsChildDropBlocked(hasDirectSimilarDescendant(node, sourceTarget.node));
          } else {
            setIsChildDropBlocked(null);
          }
        } else {
          setIsChildDropBlocked(null);
        }
      },
      onDrop() {
        setIsChildDropBlocked(null);
      },
    });
  }, [id, isLastChild, isRootNode, node, parentNode, dropTargetListRef]);

  return { isChildDropBlocked };
};
