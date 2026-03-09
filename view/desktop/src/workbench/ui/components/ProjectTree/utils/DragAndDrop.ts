import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { ListProjectResourceItem } from "@repo/ipc";

import { ProjectDragType } from "../constants";
import { LocationResourcesListData } from "../ResourcesTree/dnd/types.dnd";
import { DraggedResourceNode, DropNode, ResourceNode } from "../types";

//source
export const getSourceProjectTreeNodeData = (source: ElementDragPayload): DraggedResourceNode | null => {
  if (source.data.type !== ProjectDragType.NODE) {
    return null;
  }

  return source.data.data as DraggedResourceNode;
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
    ...(location.current.dropTargets[0].data.data as DraggedResourceNode),
    "instruction": instruction ?? undefined,
  };
};

export const getInstructionFromLocation = (location: DragLocationHistory): Instruction | null => {
  if (location.current.dropTargets.length === 0) return null;
  return extractInstruction(location.current.dropTargets[0].data);
};

export const getFirstDropTargetType = (location: DragLocationHistory): ProjectDragType | null => {
  if (location.current.dropTargets.length === 0) return null;
  return location.current.dropTargets[0].data.type as ProjectDragType;
};

export const getLocationResourcesListData = (location: DragLocationHistory): LocationResourcesListData | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== ProjectDragType.RESOURCES_LIST) return null;

  return location.current.dropTargets[0].data as LocationResourcesListData;
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

  const sortedResult = result.sort((a, b) => a.path.segments.length - b.path.segments.length);

  return sortedResult;
};
