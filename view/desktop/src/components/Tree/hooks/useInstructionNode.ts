import { RefObject, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { TreeNodeProps } from "../types";
import { canDropNode, getActualDropSourceTarget, getActualDropTargetWithInstruction } from "../utils";

export const useInstructionNode = (
  node: TreeNodeProps,
  treeId: string | number,
  dropTargetListRef: RefObject<HTMLButtonElement>,
  isLastChild: boolean,
  setPreview: React.Dispatch<React.SetStateAction<HTMLElement | null>>
) => {
  const [instruction, setInstruction] = useState<Instruction | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [canDrop, setCanDrop] = useState<boolean | null>(null);

  useEffect(() => {
    const element = dropTargetListRef.current;
    if (!element) return;

    return combine(
      draggable({
        element,
        getInitialData: () => ({
          type: "TreeNode",
          data: {
            node,
            treeId,
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
        getData: ({ input, element }) => {
          const data = { type: "TreeNode", data: { treeId, node } };

          const isReorderBeforeAvailable = true;
          let isReorderAfterAvailable = true;
          let isCombineAvailable = true;

          if (node.isFolder) {
            if (!isLastChild && !node.isExpanded) {
              isReorderAfterAvailable = false;
            }
            if (node.isExpanded) {
              isReorderAfterAvailable = false;
            }
          } else {
            isCombineAvailable = false;

            if (!isLastChild) {
              isReorderAfterAvailable = false;
            }
          }

          return attachInstruction(data, {
            input,
            element,
            operations: {
              "reorder-before": isReorderBeforeAvailable ? "available" : "not-available",
              "reorder-after": isReorderAfterAvailable ? "available" : "not-available",
              combine: isCombineAvailable ? "available" : "not-available",
            },
          });
        },
        canDrop({ source }) {
          return source.data.type === "TreeNode";
        },
        onDrag({ location, source, self }) {
          const sourceTarget = getActualDropSourceTarget(source);
          const { dropTarget } = getActualDropTargetWithInstruction(location, self);

          const instruction: Instruction | null = extractInstruction(self.data);
          setInstruction(instruction);
          setCanDrop(canDropNode(sourceTarget, dropTarget, node));
        },
        onDropTargetChange() {
          setInstruction(null);
          setCanDrop(null);
        },
        onDrop({ location, source, self }) {
          setInstruction(null);
          setCanDrop(null);

          if (location.current?.dropTargets.length === 0 || location.current.dropTargets[0].data.type !== "TreeNode") {
            return;
          }

          const sourceTarget = getActualDropSourceTarget(source);
          const { dropTarget, instruction } = getActualDropTargetWithInstruction(location, self);

          if (dropTarget?.node.uniqueId !== node.uniqueId) {
            return;
          }

          if (canDropNode(sourceTarget, dropTarget, node)) {
            window.dispatchEvent(
              new CustomEvent("moveTreeNode", {
                detail: {
                  source: sourceTarget,
                  target: dropTarget,
                  instruction,
                },
              })
            );
          }
        },
      })
    );
  }, [dropTargetListRef, isLastChild, node, setPreview, treeId]);

  return { instruction, isDragging, canDrop };
};
