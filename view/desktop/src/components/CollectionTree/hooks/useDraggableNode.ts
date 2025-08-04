import { RefObject, useContext, useEffect, useState } from "react";

import {
  attachInstruction,
  Availability,
  extractInstruction,
  Instruction,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { TreeContext } from "../Tree";
import { DragNode, DropNode, TreeCollectionNode } from "../types";
import { getLocationTreeNodeData, getSourceTreeNodeData, hasDescendant, hasDirectSimilarDescendant } from "../utils";

interface UseDraggableNodeProps {
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
  triggerRef: RefObject<HTMLButtonElement>;
  dropTargetListRef: RefObject<HTMLLIElement>;
  isLastChild: boolean;
  isRootNode: boolean;
  setPreview: React.Dispatch<React.SetStateAction<HTMLElement | null>>;
}

export const useDraggableNode = ({
  node,
  parentNode,
  triggerRef,
  dropTargetListRef,
  isLastChild,
  isRootNode,
  setPreview,
}: UseDraggableNodeProps) => {
  const { repository, id } = useContext(TreeContext);

  const [instruction, setInstruction] = useState<Instruction | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [isChildDropBlocked, setIsChildDropBlocked] = useState<boolean | null>(null);

  useEffect(() => {
    const element = triggerRef.current;
    const dropTargetListElement = dropTargetListRef.current;

    if (!element || !dropTargetListElement) return;

    return combine(
      draggable({
        element,
        canDrag() {
          return !isRootNode;
        },
        getInitialData: () => ({
          type: "TreeNode",
          data: {
            collectionId: id,
            repository,
            node,
            parentNode,
          },
        }),
        onDragStart() {
          setIsDragging(true);
        },
        onDrop: () => {
          setPreview(null);
          setIsDragging(false);
        },
        onGenerateDragPreview({ nativeSetDragImage }) {
          setCustomNativeDragPreview({
            nativeSetDragImage,
            render({ container }) {
              setPreview((prev) => (prev === container ? prev : container));
            },
          });
        },
      }),
      dropTargetForElements({
        element,
        getData: ({ input, element, source }) => {
          const data = {
            type: "TreeNode",
            data: {
              collectionId: id,
              repository,
              node,
              parentNode,
            },
          };

          const sourceTarget = getSourceTreeNodeData(source);

          if (!sourceTarget) {
            return attachInstruction(data, {
              input,
              element,
              operations: {
                "reorder-before": "not-available",
                "reorder-after": "not-available",
                combine: "not-available",
              },
            });
          }

          return attachInstruction(data, {
            input,
            element,
            operations: {
              "reorder-before": isRootNode ? "not-available" : isReorderAvailable(sourceTarget, data.data),
              "reorder-after":
                isRootNode || (node.kind === "Dir" && node.expanded)
                  ? "not-available"
                  : isReorderAvailable(sourceTarget, data.data),
              combine: isCombineAvailable(sourceTarget, data.data),
            },
          });
        },
        canDrop({ source }) {
          return source.data.type === "TreeNode";
        },
        onDrag({ location, source, self }) {
          const sourceTarget = getSourceTreeNodeData(source);
          const dropTarget = getLocationTreeNodeData(location);
          const instruction: Instruction | null = extractInstruction(self.data);

          if (!sourceTarget || !dropTarget || !instruction) {
            return;
          }

          setInstruction(instruction);
        },

        onDragLeave() {
          setInstruction(null);
        },
        onDrop() {
          setInstruction(null);
        },
      }),
      dropTargetForElements({
        element: dropTargetListElement,
        getData: () => ({
          type: "TreeNode",
          data: {
            collectionId: id,
            repository,
            node,
            parentNode,
          },
        }),
        onDrag: ({ source, location }) => {
          const sourceTarget = getSourceTreeNodeData(source);
          const dropTarget = getLocationTreeNodeData(location);

          if (!sourceTarget || !dropTarget) {
            return;
          }

          if (sourceTarget.node.id === node.id) {
            if (hasDescendant(sourceTarget.node, dropTarget.node)) {
              setIsChildDropBlocked(true);
              return;
            }
          }

          if (dropTarget.parentNode.id === node.id && dropTarget.instruction?.operation !== "combine") {
            setIsChildDropBlocked(hasDirectSimilarDescendant(node, sourceTarget.node));
            return;
          }

          setIsChildDropBlocked(null);
        },
        onDragLeave: () => {
          setIsChildDropBlocked(null);
        },
        onDrop: () => {
          setIsChildDropBlocked(null);
        },
      })
    );
  }, [dropTargetListRef, id, instruction, isRootNode, node, parentNode, repository, setPreview, triggerRef]);

  return { instruction, isDragging, isChildDropBlocked };
};

const isReorderAvailable = (sourceTarget: DragNode, dropTarget: DropNode): Availability => {
  if (sourceTarget.node.class !== dropTarget.node.class) {
    // console.log("can't drop: class mismatch");
    return "not-available";
  }

  if (sourceTarget.node.id === dropTarget.node.id) {
    // console.log("can't drop: id mismatch");
    return "not-available";
  }

  if (hasDescendant(sourceTarget.node, dropTarget.node)) {
    // console.log("can't drop: has direct descendant");
    return "blocked";
  }

  if (hasDirectSimilarDescendant(dropTarget.parentNode, sourceTarget.node)) {
    // console.log("can't drop: has direct similar descendant");
    return "blocked";
  }

  return "available";
};

const isCombineAvailable = (sourceTarget: DragNode, dropTarget: DropNode): Availability => {
  if (dropTarget.node.kind !== "Dir") {
    return "not-available";
  }

  if (hasDescendant(sourceTarget.node, dropTarget.node)) {
    // console.log("can't drop: has direct descendant");
    return "blocked";
  }

  if (hasDirectSimilarDescendant(dropTarget.node, sourceTarget.node)) {
    // console.log("can't drop: has direct similar descendant");
    return "blocked";
  }

  return "available";
};
