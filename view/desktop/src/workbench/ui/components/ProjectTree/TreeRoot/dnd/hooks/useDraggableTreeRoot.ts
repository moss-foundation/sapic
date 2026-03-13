import { RefObject, useContext, useEffect, useState } from "react";

import { attachInstruction, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ProjectDragType } from "../../../constants";
import { ProjectTreeContext } from "../../../ProjectTreeContext";
import { ProjectTree } from "../../../types";
import { isSourceTreeRoot } from "../../../utils";
import { getTreeRootSourceData } from "../getters/getTreeRootSourceData";
import { DragTreeRootData } from "../types.dnd";

interface UseDraggableTreeRootProps {
  nodeRef: RefObject<HTMLUListElement | null>;
  headerRef: RefObject<HTMLLIElement | null>;
  node: ProjectTree;
  isRenamingTreeRoot: boolean;
}

export const useDraggableTreeRoot = ({ nodeRef, headerRef, node, isRenamingTreeRoot }: UseDraggableTreeRootProps) => {
  const { id, displayMode } = useContext(ProjectTreeContext);

  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [instruction, setInstruction] = useState<Instruction | null>(null);

  useEffect(() => {
    const headerElement = headerRef.current;
    const nodeElement = nodeRef.current;

    if (!headerElement || !nodeElement || isRenamingTreeRoot) return;

    const treeRootData: DragTreeRootData = {
      type: ProjectDragType.TREE_ROOT,
      data: {
        projectId: id,
        node,
      },
    };

    return combine(
      draggable({
        element: headerElement,
        getInitialData: () => treeRootData,
        onDragStart: () => {
          setIsDragging(true);
        },
        onDrop: () => {
          setIsDragging(false);
        },
      }),
      dropTargetForElements({
        element: nodeElement,
        canDrop: ({ source }) => isSourceTreeRoot(source),
        getIsSticky: () => false,
        getData: ({ input, source }) => {
          const getSourceData = getTreeRootSourceData(source);
          const areDifferentProjects = getSourceData.data.projectId !== treeRootData.data.projectId;

          return attachInstruction(treeRootData, {
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
  }, [displayMode, id, isRenamingTreeRoot, node, nodeRef, headerRef]);

  return { isDragging, instruction };
};
