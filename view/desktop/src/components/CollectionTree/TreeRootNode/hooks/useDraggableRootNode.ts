import { RefObject, useContext, useEffect, useState } from "react";

import {
  attachInstruction,
  Availability,
  extractInstruction,
  Instruction,
  Operation,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import {
  draggable,
  dropTargetForElements,
  monitorForElements,
} from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { CollectionTreeContext } from "../../CollectionTreeContext";
import { DragNode, TreeCollectionNode, TreeCollectionRootNode } from "../../types";
import {
  getInstructionFromSelf,
  getLocationTreeCollectionData,
  getLocationTreeCollectionNodeData,
  getSourceTreeCollectionNodeData,
  hasAnotherDirectDescendantWithSimilarName,
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
      //for regular nodes
      dropTargetForElements({
        element: triggerElement,
        canDrop({ source }) {
          return isSourceTreeCollectionNode(source) && displayMode === "REQUEST_FIRST";
        },
        getData({ input, source }) {
          const dropTarget = {
            type: "TreeRootNode",
            node,
            collectionId: id,
          };

          const sourceTarget = getSourceTreeCollectionNodeData(source);
          if (sourceTarget) {
            return attachInstruction(dropTarget, {
              element: triggerElement,
              input,
              operations: evaluateTreeNodeOperations(node.requests, sourceTarget),
            });
          }

          return attachInstruction(dropTarget, {
            element: triggerElement,
            input,
            operations: {
              "reorder-before": "not-available",
              "reorder-after": "not-available",
              combine: "not-available",
            },
          });
        },
        onDragEnter: ({ self }) => {
          setInstruction(getInstructionFromSelf(self));
        },
        onDropTargetChange: () => {
          setIsChildDropBlocked(null);
        },
        onDragLeave: () => {
          setInstruction(null);
        },
        onDrop: () => {
          setInstruction(null);
        },
      }),
      //for collections
      dropTargetForElements({
        element: dirElement,
        canDrop: ({ source }) => isSourceTreeRootNode(source),
        getIsSticky: () => true,
        getData: ({ input }) => {
          const dropTarget = {
            type: "TreeCollection",
            node,
            collectionId: id,
          };

          return attachInstruction(dropTarget, {
            element: dirElement,
            input,
            operations: {
              "reorder-before": "available",
              "reorder-after": "available",
              combine: "not-available",
            },
          });
        },
        onDrag: ({ source, location, self }) => {
          const sourceTarget = getTreeRootNodeSourceData(source);
          const dropTarget = getLocationTreeCollectionData(location);
          const instruction = extractInstruction(self.data);

          if (!sourceTarget || !dropTarget) {
            setInstruction(null);
            return;
          }

          setInstruction(instruction);
        },
        onDragLeave: () => {
          setIsChildDropBlocked(null);
          setInstruction(null);
        },
        onDrop: () => {
          setIsChildDropBlocked(null);
          setInstruction(null);
        },
      }),
      //for checking if child drop is blocked
      monitorForElements({
        canMonitor: ({ source }) => isSourceTreeCollectionNode(source) && displayMode === "REQUEST_FIRST",
        onDrag: ({ source, location }) => {
          const dropTarget = getLocationTreeCollectionNodeData(location);
          const sourceTarget = getSourceTreeCollectionNodeData(source);

          if (!dropTarget || !sourceTarget) {
            setIsChildDropBlocked(null);
            return;
          }

          if (displayMode === "REQUEST_FIRST") {
            if (dropTarget?.parentNode.id === node.requests.id && dropTarget.instruction?.operation !== "combine") {
              setIsChildDropBlocked(hasAnotherDirectDescendantWithSimilarName(node.requests, sourceTarget.node));
              return;
            }
          }

          setIsChildDropBlocked(null);
        },
        onDrop: () => {
          setIsChildDropBlocked(null);
        },
        onDropTargetChange: () => {
          setIsChildDropBlocked(null);
        },
      })
    );
  }, [dirRef, displayMode, id, isRenamingNode, node, triggerRef]);

  return { isDragging, isChildDropBlocked, instruction };
};

export const evaluateTreeNodeOperations = (
  dropTarget: TreeCollectionNode,
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
