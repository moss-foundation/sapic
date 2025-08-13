import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { CollectionTreeContext } from "../../CollectionTreeContext";
import { TreeCollectionNode } from "../../types";
import {
  evaluateIsChildDropBlocked,
  getLocationTreeCollectionNodeData,
  getSourceTreeCollectionNodeData,
  hasDescendant,
  isCombineAvailable,
  isReorderAvailable,
  isSourceTreeCollectionNode,
} from "../../utils";

interface UseDraggableNodeProps {
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
  triggerRef: RefObject<HTMLDivElement>;
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
  const { repository, id } = useContext(CollectionTreeContext);

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
          type: "TreeCollectionNode",
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
            type: "TreeCollectionNode",
            data: {
              collectionId: id,
              repository,
              node,
              parentNode,
            },
          };

          const sourceTarget = getSourceTreeCollectionNodeData(source);

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
          return isSourceTreeCollectionNode(source);
        },
        onDrag({ location, source, self }) {
          const sourceTarget = getSourceTreeCollectionNodeData(source);
          const dropTarget = getLocationTreeCollectionNodeData(location);
          const instruction: Instruction | null = extractInstruction(self.data);

          if (!sourceTarget || !dropTarget || !instruction) {
            setIsChildDropBlocked(null);
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
          type: "TreeCollectionNode",
          data: {
            collectionId: id,
            repository,
            node,
            parentNode,
          },
        }),
        onDrag: ({ source, location }) => {
          const sourceTarget = getSourceTreeCollectionNodeData(source);
          const dropTarget = getLocationTreeCollectionNodeData(location);

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
            const isChildDropBlocked = evaluateIsChildDropBlocked(node, sourceTarget.node);
            setIsChildDropBlocked(isChildDropBlocked);
            return;
          }

          setIsChildDropBlocked(null);
        },
        onDropTargetChange: () => {
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
