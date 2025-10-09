import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { ProjectDragType } from "../../constants";
import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeNode, ProjectTreeRootNode } from "../../types";
import {
  evaluateIsChildDropBlocked,
  getLocationProjectTreeNodeData,
  getSourceProjectTreeNodeData,
  hasDescendant,
  isCombineAvailable,
  isReorderAvailable,
  isSourceProjectTreeNode,
} from "../../utils";

interface UseDraggableNodeProps {
  node: ProjectTreeNode;
  parentNode: ProjectTreeNode | ProjectTreeRootNode;
  triggerRef: RefObject<HTMLDivElement | null>;
  dropTargetListRef: RefObject<HTMLLIElement | null>;
  isLastChild: boolean;
  setPreview: React.Dispatch<React.SetStateAction<HTMLElement | null>>;
}

export const useDraggableNode = ({
  node,
  parentNode,
  triggerRef,
  dropTargetListRef,
  setPreview,
}: UseDraggableNodeProps) => {
  const { id } = useContext(ProjectTreeContext);

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
        getInitialData: () => ({
          type: ProjectDragType.NODE,
          data: {
            projectId: id,
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
            type: ProjectDragType.NODE,
            data: {
              projectId: id,
              node,
              parentNode,
            },
          };

          const sourceTarget = getSourceProjectTreeNodeData(source);
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
              "reorder-before": isReorderAvailable(sourceTarget, data.data),
              "reorder-after": isReorderAvailable(sourceTarget, data.data),
              combine: isCombineAvailable(sourceTarget, data.data),
            },
          });
        },
        canDrop({ source }) {
          return isSourceProjectTreeNode(source);
        },
        onDrag({ location, source, self }) {
          const sourceTarget = getSourceProjectTreeNodeData(source);
          const dropTarget = getLocationProjectTreeNodeData(location);
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
          type: ProjectDragType.NODE,
          data: {
            projectId: id,
            node,
            parentNode,
          },
        }),
        onDrag: ({ source, location }) => {
          const sourceTarget = getSourceProjectTreeNodeData(source);
          const dropTarget = getLocationProjectTreeNodeData(location);

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
  }, [dropTargetListRef, id, instruction, node, parentNode, setPreview, triggerRef]);

  return { instruction, isDragging, isChildDropBlocked };
};
