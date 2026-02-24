import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectDragType } from "../../constants";
import { ProjectTreeContext } from "../../ProjectTreeContext";
import { ProjectTreeRootNode } from "../../types";
import { isSourceTreeRootNode } from "../../utils";
import { getTreeRootNodeSourceData } from "../getters/getTreeRootNodeSourceData";
import { DragTreeRootNodeData } from "../types.dnd";

interface UseDraggableRootNodeProps {
  nodeRef: RefObject<HTMLUListElement | null>;
  triggerRef: RefObject<HTMLLIElement | null>;
  node: ProjectTreeRootNode;
  isRenamingNode: boolean;
}

export const useDraggableRootNode = ({ nodeRef, triggerRef, node, isRenamingNode }: UseDraggableRootNodeProps) => {
  const { id, displayMode } = useContext(ProjectTreeContext);

  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const triggerElement = triggerRef.current;
    const nodeElement = nodeRef.current;

    if (!triggerElement || !nodeElement || isRenamingNode) return;

    const rootNodeData: DragTreeRootNodeData = {
      type: ProjectDragType.ROOT_NODE,
      data: {
        projectId: id,
        node,
      },
    };

    return combine(
      draggable({
        element: triggerElement,
        getInitialData: () => rootNodeData,
        onDragStart: () => setIsDragging(true),
        onDrop: () => setIsDragging(false),
      }),
      dropTargetForElements({
        element: nodeElement,
        canDrop: ({ source }) => isSourceTreeRootNode(source),
        getIsSticky: () => false,
        getData: ({ input, source }) => {
          const getSourceData = getTreeRootNodeSourceData(source);
          const areDifferentProjects = getSourceData.data.projectId !== rootNodeData.data.projectId;

          return attachInstruction(rootNodeData, {
            element: nodeElement,
            input,
            operations: {
              "reorder-before": areDifferentProjects ? "available" : "not-available",
              "reorder-after": areDifferentProjects ? "available" : "not-available",
              combine: "not-available",
            },
          });
        },
        onDrag: ({ self }) => {
          const instruction = extractInstruction(self.data);
          setInstruction(instruction);
        },
        onDragLeave: () => {
          setInstruction(null);
        },
        onDrop: () => {
          setInstruction(null);
        },
      })
    );
  }, [displayMode, id, isRenamingNode, node, nodeRef, triggerRef]);

  return { isDragging, instruction };
};
