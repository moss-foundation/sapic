import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { ProjectDragType } from "../../../constants";
import { ProjectTreeContext } from "../../../ProjectTreeContext";
import { ResourcesTreeRoot } from "../../../TreeRoot/types";
import { getLocationProjectTreeNodeData, getSourceProjectTreeNodeData, isSourceProjectTreeNode } from "../../../utils";
import { ResourceNode } from "../../types";
import { DragResourceNode } from "../types.dnd";
import { isNodeCombineAvailable } from "../validation/isNodeCombineAvailable";
import { isNodeReorderAvailable } from "../validation/isNodeReorderAvailable";

interface UseDraggableResourceNodeProps {
  node: ResourceNode;
  parentNode: ResourceNode | ResourcesTreeRoot;
  triggerRef: RefObject<HTMLDivElement | null>;
  setPreview: React.Dispatch<React.SetStateAction<HTMLElement | null>>;
}

export const useDraggableResourceNode = ({
  node,
  parentNode,
  triggerRef,
  setPreview,
}: UseDraggableResourceNodeProps) => {
  const { id } = useContext(ProjectTreeContext);

  const [instruction, setInstruction] = useState<Instruction | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);

  useEffect(() => {
    const element = triggerRef.current;

    if (!element) return;

    return combine(
      draggable({
        element,
        getInitialData: (): DragResourceNode => ({
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
          const sourceData = getSourceProjectTreeNodeData(source);
          const locationData: DragResourceNode = {
            type: ProjectDragType.NODE,
            data: {
              projectId: id,
              node,
              parentNode,
            },
          };

          return attachInstruction(locationData, {
            input,
            element,
            operations: {
              "reorder-before": isNodeReorderAvailable(sourceData, locationData, "reorder-before"),
              "reorder-after": isNodeReorderAvailable(sourceData, locationData, "reorder-after"),
              combine: isNodeCombineAvailable(sourceData, locationData),
            },
          });
        },
        canDrop({ source }) {
          return isSourceProjectTreeNode(source);
        },
        onDrag({ location, source, self }) {
          const sourceTarget = getSourceProjectTreeNodeData(source);
          const locationTarget = getLocationProjectTreeNodeData(location);
          const instruction: Instruction | null = extractInstruction(self.data);

          if (!sourceTarget || !locationTarget || !instruction) {
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
