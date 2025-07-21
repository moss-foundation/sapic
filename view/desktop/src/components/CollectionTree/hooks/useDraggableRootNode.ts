import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import {
  BaseEventPayload,
  DropTargetLocalizedData,
  ElementDragType,
} from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { TreeContext } from "../Tree";
import { TreeCollectionRootNode } from "../types";
import { isSourceTreeNode, isSourceTreeRootNode } from "../utils2";

export const useDraggableRootNode = (
  draggableRef: RefObject<HTMLDivElement>,
  node: TreeCollectionRootNode,
  isRenamingNode: boolean
) => {
  const { id, displayMode } = useContext(TreeContext);

  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [canDrop, setCanDrop] = useState<boolean | null>(false);
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const element = draggableRef.current;
    if (!element || isRenamingNode) return;

    const evaluateInstruction = ({ self, source }: BaseEventPayload<ElementDragType> & DropTargetLocalizedData) => {
      const instruction: Instruction | null = extractInstruction(self.data);

      if (isSourceTreeRootNode(source)) {
        if (instruction?.operation === "combine") {
          setCanDrop(null);
          setInstruction(null);
        } else {
          setCanDrop(true);
          setInstruction(instruction);
        }
      }

      if (isSourceTreeNode(source)) {
        if (instruction?.operation === "combine") {
          setCanDrop(true);
          setInstruction(instruction);
        } else {
          setCanDrop(null);
          setInstruction(null);
        }
      }
    };
    return combine(
      draggable({
        element,
        getInitialData: () => ({
          type: "TreeRootNode",
          data: {
            node,
            collectionId: id,
          },
        }),
        onDragStart() {
          setIsDragging(true);
        },
        onDrop() {
          setIsDragging(false);
        },
      }),
      dropTargetForElements({
        element,
        getData({ input }) {
          return attachInstruction(
            {
              node,
              collectionId: id,
            },
            {
              element,
              input,
              operations: {
                "reorder-before": "available",
                "reorder-after": "available",
                combine: displayMode === "REQUEST_FIRST" ? "available" : "not-available",
              },
            }
          );
        },
        canDrop({ source }) {
          return source.data.type === "TreeRootNode" || source.data.type === "TreeNode";
        },
        getIsSticky() {
          return true;
        },
        onDragStart: evaluateInstruction,
        onDragEnter: evaluateInstruction,
        onDrag: evaluateInstruction,
        onDragLeave() {
          setInstruction(null);
        },
        onDrop() {
          setInstruction(null);
        },
      })
    );
  }, [node, isRenamingNode, draggableRef, id, displayMode]);

  return {
    canDrop,
    instruction,
    setInstruction,
    isDragging,
  };
};
