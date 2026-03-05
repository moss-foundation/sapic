import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import {
  DragLocationHistory,
  DropTargetRecord,
  ElementDragPayload,
} from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { ListProjectResourceItem } from "@repo/ipc";

import { ProjectDragType } from "../constants";
import { DragNode, DropNode, ResourceNode } from "../types";

//source
export const getSourceProjectTreeNodeData = (source: ElementDragPayload): DragNode | null => {
  if (source.data.type !== ProjectDragType.NODE) {
    return null;
  }

  return source.data.data as DragNode;
};

export const isSourceProjectTreeNode = (source: ElementDragPayload): boolean => {
  return source.data.type === ProjectDragType.NODE;
};

//location
export const getLocationProjectTreeNodeData = (location: DragLocationHistory): DropNode | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== ProjectDragType.NODE) return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    ...(location.current.dropTargets[0].data.data as DragNode),
    "instruction": instruction ?? undefined,
  };
};

export const getInstructionFromLocation = (location: DragLocationHistory): Instruction | null => {
  if (location.current.dropTargets.length === 0) return null;
  return extractInstruction(location.current.dropTargets[0].data);
};

//other checks

export const getAllNestedResources = (node: ResourceNode): ListProjectResourceItem[] => {
  const result: ListProjectResourceItem[] = [];

  result.push({
    id: node.id,
    name: node.name,
    kind: node.kind,
    class: node.class,
    path: node.path,
    protocol: node.protocol,
  });

  for (const child of node.childNodes) {
    result.push(...getAllNestedResources(child));
  }

  return result;
};

export const getInstructionFromSelf = (self: DropTargetRecord): Instruction | null => {
  return extractInstruction(self.data);
};
