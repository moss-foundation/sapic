import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DragNode, DropNode } from "../../../types";
import { NodeDropOperation } from "../constants";

interface CalculateNodeDropOperationProps {
  sourceTreeNodeData: DragNode | null;
  locationTreeNodeData: DropNode | null;
  instruction: Instruction | null;
}

export const calculateNodeDropOperation = ({
  sourceTreeNodeData,
  locationTreeNodeData,
  instruction,
}: CalculateNodeDropOperationProps): NodeDropOperation | null => {
  if (!sourceTreeNodeData || !instruction || !locationTreeNodeData) {
    console.warn("can't drop: no source, instruction, or location", {
      sourceTreeNodeData,
      locationTreeNodeData,
      instruction,
    });

    return null;
  }

  if (instruction.blocked) {
    return null;
  }

  const isSameProject = sourceTreeNodeData.projectId === locationTreeNodeData.projectId;
  if (isSameProject) {
    if (instruction.operation === "combine") {
      return NodeDropOperation.NODE_ON_FOLDER_WITHIN_PROJECT;
    } else {
      return NodeDropOperation.NODE_ON_NODE_WITHIN_PROJECT;
    }
  } else {
    if (instruction.operation === "combine") {
      return NodeDropOperation.NODE_ON_FOLDER_TO_ANOTHER_PROJECT;
    } else {
      return NodeDropOperation.NODE_ON_NODE_TO_ANOTHER_PROJECT;
    }
  }
};
