import { RefObject, useContext, useEffect, useState } from "react";

import {
  attachInstruction,
  Availability,
  extractInstruction,
  Instruction,
  Operation,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { CollectionTreeContext } from "../../CollectionTreeContext";
import { DragNode, TreeCollectionRootNode } from "../../types";
import {
  getLocationTreeCollectionData,
  getLocationTreeRootNodeData,
  getSourceTreeCollectionNodeData,
  hasDirectSimilarDescendant,
  isSourceTreeCollectionNode,
  isSourceTreeRootNode,
} from "../../utils";
import { getTreeRootNodeSourceData } from "../../utils/TreeRoot";

interface UseDraggableRootNodeProps {
  dirRef: RefObject<HTMLUListElement | null>;
  triggerRef: RefObject<HTMLLIElement | null>;
  node: TreeCollectionRootNode;
  isRenamingNode: boolean;
}

export const useDraggableRootNode = ({ dirRef, triggerRef, node, isRenamingNode }: UseDraggableRootNodeProps) => {
  const { id, displayMode } = useContext(CollectionTreeContext);

  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [instruction, setInstruction] = useState<Instruction | null>(null);
  const [dirInstruction, setDirInstruction] = useState<Instruction | null>(null);
  const [isChildDropBlocked, setIsChildDropBlocked] = useState<boolean | null>(null);

  useEffect(() => {
    const triggerElement = triggerRef.current;
    const dirElement = dirRef.current;

    if (!triggerElement || !dirElement || isRenamingNode) return;

    return combine(
      draggable({
        element: triggerElement,
        getInitialData: () => ({
          type: "TreeRootNode",
          data: {
            node,
            collectionId: id,
          },
        }),
        onDragStart: () => setIsDragging(true),
        onDrop: () => setIsDragging(false),
      }),
      dropTargetForElements({
        element: dirElement,
        canDrop: ({ source }) => isSourceTreeRootNode(source) || isSourceTreeCollectionNode(source),
        getIsSticky: ({ source }) => isSourceTreeRootNode(source),
        getData: ({ input, source }) => {
          const dropTarget = {
            type: "TreeRootNode",
            node,
            collectionId: id,
          };

          if (isSourceTreeRootNode(source)) {
            return attachInstruction(dropTarget, {
              element: dirElement,
              input,
              operations: {
                "reorder-before": "available",
                "reorder-after": "available",
                combine: "not-available",
              },
            });
          }

          if (isSourceTreeCollectionNode(source)) {
            const sourceTarget = getSourceTreeCollectionNodeData(source);
            if (sourceTarget) {
              return attachInstruction(dropTarget, {
                element: dirElement,
                input,
                operations: evaluateTreeNodeOperations(node, sourceTarget),
              });
            }
          }

          return attachInstruction(dropTarget, {
            element: dirElement,
            input,
            operations: {
              "reorder-before": "not-available",
              "reorder-after": "not-available",
              combine: "not-available",
            },
          });
        },
        onDrag: ({ source, location, self }) => {
          const sourceTarget = getTreeRootNodeSourceData(source);
          const dropTarget = getLocationTreeCollectionData(location);
          const rootDropTarget = getLocationTreeRootNodeData(location);
          const instruction = extractInstruction(self.data);

          if (!sourceTarget) {
            setInstruction(null);
            setDirInstruction(null);
            return;
          }

          if (!rootDropTarget && !dropTarget) {
            setDirInstruction(null);
            setInstruction(null);
            return;
          }

          if (rootDropTarget) {
            setInstruction(null);
            setDirInstruction(instruction);
          }

          if (dropTarget) {
            setInstruction(instruction);
            setDirInstruction(null);
          }
        },
        onDragLeave: () => {
          setIsChildDropBlocked(null);
          setInstruction(null);
          setDirInstruction(null);
        },
        onDrop: () => {
          setIsChildDropBlocked(null);
          setInstruction(null);
          setDirInstruction(null);
        },
      })
    );
  }, [dirRef, displayMode, id, isRenamingNode, node, triggerRef]);

  return { isDragging, isChildDropBlocked, instruction, dirInstruction };
};

export const evaluateTreeNodeOperations = (
  dropTarget: TreeCollectionRootNode,
  sourceNode: DragNode
): {
  [TKey in Operation]?: Availability;
} => {
  const hasSimilarDescendant = hasDirectSimilarDescendant(dropTarget, sourceNode.node);

  return {
    "reorder-before": "not-available",
    "reorder-after": "not-available",
    combine: hasSimilarDescendant ? "blocked" : "available",
  };
};
