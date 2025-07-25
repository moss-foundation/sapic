import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragNode, DropNode, DropRootNode } from "../types";

export const getSourceTreeNodeData = (source: ElementDragPayload): DragNode | null => {
  if (source.data.type !== "TreeNode") {
    return null;
  }

  return source.data.data as DragNode;
};

export const getLocationTreeNodeData = (location: DragLocationHistory): DropNode | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== "TreeNode") return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    ...(location.current.dropTargets[0].data.data as DragNode),
    "instruction": instruction ?? undefined,
  };
};

export const getLocationTreeRootNodeData = (location: DragLocationHistory): DropRootNode | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== "TreeRootNode") return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    ...(location.current.dropTargets[0].data as unknown as DropRootNode),
    "instruction": instruction ?? undefined,
  };
};
