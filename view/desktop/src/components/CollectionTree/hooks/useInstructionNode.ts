import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";
import { canDropNode, getLocationTreeNodeData, getSourceTreeNodeData } from "../utils";

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
        onDrag({ location, source, self }) {
          const sourceTarget = getSourceTreeNodeData(source);
          const dropTarget = getLocationTreeNodeData(location);

          if (!sourceTarget || !dropTarget) {
            return;
          }

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
