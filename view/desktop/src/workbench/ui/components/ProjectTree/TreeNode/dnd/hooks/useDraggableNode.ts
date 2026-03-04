import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { ProjectDragType } from "../../../constants";
import { ProjectTreeContext } from "../../../ProjectTreeContext";
import { ProjectTreeNode, ProjectTreeRootNode } from "../../../types";
import {
  getLocationProjectTreeNodeData,
  getSourceProjectTreeNodeData,
  isCombineAvailable,
  isSourceProjectTreeNode,
} from "../../../utils";
import { isNodeReorderAvailable } from "../validation/isNodeReorderAvailable";

interface UseDraggableNodeProps {
  node: ProjectTreeNode;
  parentNode: ProjectTreeNode | ProjectTreeRootNode;
  triggerRef: RefObject<HTMLDivElement | null>;
  setPreview: React.Dispatch<React.SetStateAction<HTMLElement | null>>;
}

export const useDraggableNode = ({ node, parentNode, triggerRef, setPreview }: UseDraggableNodeProps) => {
  const { id } = useContext(ProjectTreeContext);

  const [instruction, setInstruction] = useState<Instruction | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);

  useEffect(() => {
    const element = triggerRef.current;

    if (!element) return;

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

          //TODO make it shorter
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
              "reorder-before": isNodeReorderAvailable(sourceTarget, data.data),
              "reorder-after": isNodeReorderAvailable(sourceTarget, data.data),
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
      })
    );
  }, [id, triggerRef, node, parentNode, setPreview]);

  return { instruction, isDragging };
};
