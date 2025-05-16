import { RefObject, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { TreeNodeProps } from "../types";
import { canDropNode, getActualDropSourceTarget, getActualDropTargetWithInstruction } from "../utils";

export const useInstructionNode = (
  node: TreeNodeProps,
  treeId: string | number,
  dropTargetListRef: RefObject<HTMLButtonElement>,
  depth: number,
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

          const block: Instruction["type"][] = [];

          if (node.isFolder) {
            if (!isLastChild && !node.isExpanded) {
              block.push("reorder-below");
            }
          } else {
            block.push("make-child");

            if (isLastChild) {
              block.push("reorder-below");
            }
          }

          return attachInstruction(data, {
            input,
            element,
            indentPerLevel: 1,
            currentLevel: depth,
            mode: "standard",
            block,
          });
        },
        canDrop({ source }) {
          return source.data.type === "TreeNode";
        },
        onDrag({ location, source }) {
          const sourceTarget = getActualDropSourceTarget(source);
          const { dropTarget } = getActualDropTargetWithInstruction(location);

          setInstruction(extractInstruction(location.current.dropTargets[0].data));
          setCanDrop(canDropNode(sourceTarget, dropTarget, node));
        },
        onDragLeave() {
          setInstruction(null);
          setCanDrop(null);
        },
        onDrop({ location, source }) {
          setInstruction(null);
          setCanDrop(null);

          if (location.current?.dropTargets.length === 0 || location.current.dropTargets[0].data.type !== "TreeNode") {
            return;
          }

          const sourceTarget = getActualDropSourceTarget(source);
          const { dropTarget, instruction } = getActualDropTargetWithInstruction(location);

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
  }, [dropTargetListRef, node, treeId]);

  return { instruction, isDragging, canDrop };
};
