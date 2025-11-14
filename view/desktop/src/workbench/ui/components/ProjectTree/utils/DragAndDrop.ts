import {
  Availability,
  extractInstruction,
  Instruction,
  Operation,
} from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import {
  DragLocationHistory,
  DropTargetRecord,
  ElementDragPayload,
} from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { StreamResourcesEvent } from "@repo/moss-project";

import { ProjectDragType } from "../constants";
import { DragNode, DropNode, DropRootNode, ProjectTreeNode } from "../types";
import { hasDescendant, hasDirectDescendantWithSimilarName, hasDirectSimilarDescendant } from "./TreeProjectNode";

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

export const getLocationProjectTreeData = (location: DragLocationHistory): DropNode | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== ProjectDragType.ROOT_NODE) return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    ...(location.current.dropTargets[0].data.data as DragNode),
    "instruction": instruction ?? undefined,
  };
};

export const getAllNestedLocationProjectTreeData = (location: DragLocationHistory): DropNode[] => {
  if (location.current.dropTargets.length === 0) return [];
  return location.current.dropTargets.map((target) => {
    return {
      ...(target.data.data as DragNode),
      "instruction": extractInstruction(target.data) ?? undefined,
    };
  }) as unknown as DropNode[];
};

export const getLocationProjectTreeRootNodeData = (location: DragLocationHistory): DropRootNode | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== ProjectDragType.ROOT_NODE) return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    ...(location.current.dropTargets[0].data as unknown as DropRootNode),
    "instruction": instruction ?? undefined,
  };
};

export const getInstructionFromLocation = (location: DragLocationHistory): Instruction | null => {
  return extractInstruction(location.current.dropTargets[0].data);
};

//other checks
export const doesLocationHasProjectTreeNode = (location: DragLocationHistory): boolean => {
  if (location.current.dropTargets.length === 0) return false;
  return location.current.dropTargets[0].data.type === ProjectDragType.NODE;
};

export const getAllNestedResources = (node: ProjectTreeNode): StreamResourcesEvent[] => {
  const result: StreamResourcesEvent[] = [];

  result.push({
    id: node.id,
    name: node.name,
    kind: node.kind,
    order: node.order,
    class: node.class,
    path: node.path,
    protocol: node.protocol,
    expanded: node.expanded,
  });

  for (const child of node.childNodes) {
    result.push(...getAllNestedResources(child));
  }

  return result;
};

export const getInstructionFromSelf = (self: DropTargetRecord): Instruction | null => {
  return extractInstruction(self.data);
};

export const canDropNode = (sourceTarget: DragNode, dropTarget: DropNode, operation: Operation) => {
  if (sourceTarget.node.class !== dropTarget.node.class) {
    return false;
  }

  if (sourceTarget.node.id === dropTarget.node.id) {
    return false;
  }

  if (dropTarget.node.kind === "Item") {
    if (hasDirectDescendantWithSimilarName(dropTarget.parentNode, sourceTarget.node)) {
      return false;
    }
  }

  if (dropTarget.node.kind === "Dir") {
    if (operation === "combine") {
      if (hasDirectDescendantWithSimilarName(dropTarget.node, sourceTarget.node)) {
        return false;
      }
    } else {
      if (hasDirectDescendantWithSimilarName(dropTarget.parentNode, sourceTarget.node)) {
        return false;
      }
    }
  }

  return true;
};

//operations rules

export const isReorderAvailable = (sourceTarget: DragNode, dropTarget: DropNode): Availability => {
  if (sourceTarget.node.id === dropTarget.node.id) {
    return "not-available";
  }

  if (dropTarget.node.kind === "Dir" && dropTarget.node.expanded) {
    return "not-available";
  }

  if (sourceTarget.node.class !== dropTarget.node.class) {
    return "blocked";
  }

  if (hasDescendant(sourceTarget.node, dropTarget.node)) {
    return "blocked";
  }

  if (hasDirectDescendantWithSimilarName(dropTarget.parentNode, sourceTarget.node)) {
    return "blocked";
  }

  return "available";
};

export const isCombineAvailable = (sourceTarget: DragNode, dropTarget: DropNode): Availability => {
  if (dropTarget.node.kind !== "Dir") {
    return "not-available";
  }

  if (sourceTarget.node.id === dropTarget.node.id) {
    return "blocked";
  }

  if (sourceTarget.node.class !== dropTarget.node.class) {
    return "blocked";
  }

  if (hasDescendant(sourceTarget.node, dropTarget.node)) {
    return "blocked";
  }

  if (hasDirectSimilarDescendant(dropTarget.node, sourceTarget.node)) {
    return "blocked";
  }

  return "available";
};

export const evaluateIsChildDropBlocked = (parentNode: ProjectTreeNode, dropNode: ProjectTreeNode): boolean => {
  if (parentNode.class !== dropNode.class) {
    return true;
  }

  if (hasDirectDescendantWithSimilarName(parentNode, dropNode)) {
    return true;
  }

  return false;
};
