import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";
import { canDropNode, getActualDropSourceTarget } from "../utils";
import { getLocationTreeNodeData } from "../utils2";

export const useInstructionNode = (
  node: TreeCollectionNode,
  parentNode: TreeCollectionNode,
  collectionId: string | number,
  dropTargetListRef: RefObject<HTMLButtonElement>,
  isLastChild: boolean,
  isRootNode: boolean,
  setPreview: React.Dispatch<React.SetStateAction<HTMLElement | null>>
) => {
  const { repository, id } = useContext(TreeContext);

  const [instruction, setInstruction] = useState<Instruction | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [canDrop, setCanDrop] = useState<boolean | null>(null);

  useEffect(() => {
    const element = dropTargetListRef.current;
    if (!element) return;

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
        getData: ({ input, element }) => {
          const data = {
            type: "TreeNode",
            data: {
              collectionId,
              node,
              parentNode,
            },
          };

          let isReorderBeforeAvailable = true;
          let isReorderAfterAvailable = true;
          let isCombineAvailable = true;

          if (node.kind === "Dir") {
            if (!isLastChild && !node.expanded) {
              isReorderAfterAvailable = false;
            }
            if (node.expanded) {
              isReorderAfterAvailable = false;
            }
            if (isRootNode) {
              isReorderBeforeAvailable = false;
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
          const dropTarget = getLocationTreeNodeData(location);

          const instruction: Instruction | null = extractInstruction(self.data);

          setInstruction(instruction);
          setCanDrop(canDropNode(sourceTarget, dropTarget));
        },
        onDropTargetChange() {
          setInstruction(null);
          setCanDrop(null);
        },
        onDrop() {
          setInstruction(null);
          setCanDrop(null);
        },
      })
    );
  }, [collectionId, dropTargetListRef, id, isLastChild, isRootNode, node, parentNode, repository, setPreview]);

  return { instruction, isDragging, canDrop };
};
