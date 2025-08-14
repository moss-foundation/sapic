import { Availability, extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import {
  DragLocationHistory,
  DropTargetRecord,
  ElementDragPayload,
} from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { EntryInfo } from "@repo/moss-collection";

import { DragNode, DropNode, DropRootNode, TreeCollectionNode } from "../types";
import {
  hasAnotherDirectDescendantWithSimilarName,
  hasDescendant,
  hasDirectSimilarDescendant,
} from "./TreeCollectionNode";

//source
export const getSourceTreeCollectionNodeData = (source: ElementDragPayload): DragNode | null => {
  if (source.data.type !== "TreeCollectionNode") {
    return null;
  }

  return source.data.data as DragNode;
};

export const isSourceTreeCollectionNode = (source: ElementDragPayload): boolean => {
  return source.data.type === "TreeCollectionNode";
};

//location
export const getLocationTreeCollectionNodeData = (location: DragLocationHistory): DropNode | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== "TreeCollectionNode") return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    ...(location.current.dropTargets[0].data.data as DragNode),
    "instruction": instruction ?? undefined,
  };
};

export const getLocationTreeCollectionData = (location: DragLocationHistory): DropNode | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== "TreeCollection") return null;

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

export const getInstructionFromLocation = (location: DragLocationHistory): Instruction | null => {
  return extractInstruction(location.current.dropTargets[0].data);
};

//other checks
export const doesLocationHaveTreeCollectionNode = (location: DragLocationHistory): boolean => {
  if (location.current.dropTargets.length === 0) return false;
  return location.current.dropTargets[0].data.type === "TreeCollectionNode";
};

export const getAllNestedEntries = (node: TreeCollectionNode): EntryInfo[] => {
  const result: EntryInfo[] = [];

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
    result.push(...getAllNestedEntries(child));
  }

  return result;
};

export const getInstructionFromSelf = (self: DropTargetRecord): Instruction | null => {
  return extractInstruction(self.data);
};

export const canDropNode = (sourceTarget: DragNode, dropTarget: DropNode) => {
  if (sourceTarget.node.class !== dropTarget.node.class) {
    return false;
  }

  if (sourceTarget.node.id === dropTarget.node.id) {
    return false;
  }

  if (dropTarget.node.kind === "Dir") {
    if (hasDescendant(dropTarget.node, sourceTarget.node)) {
      return false;
    }

    if (hasAnotherDirectDescendantWithSimilarName(dropTarget.node, sourceTarget.node)) {
      return false;
    }
  }

  if (dropTarget.node.kind === "Item") {
    if (hasAnotherDirectDescendantWithSimilarName(dropTarget.parentNode, sourceTarget.node)) {
      return false;
    }
  }

  return true;
};

//operations rules

export const isReorderAvailable = (sourceTarget: DragNode, dropTarget: DropNode): Availability => {
  if (sourceTarget.node.id === dropTarget.node.id) {
    return "not-available";
  }

  if (sourceTarget.node.class !== dropTarget.node.class) {
    return "blocked";
  }

  if (hasDescendant(sourceTarget.node, dropTarget.node)) {
    return "blocked";
  }

  if (hasAnotherDirectDescendantWithSimilarName(dropTarget.parentNode, sourceTarget.node)) {
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

export const evaluateIsChildDropBlocked = (parentNode: TreeCollectionNode, dropNode: TreeCollectionNode): boolean => {
  if (parentNode.class !== dropNode.class) {
    return true;
  }

  if (hasAnotherDirectDescendantWithSimilarName(parentNode, dropNode)) {
    return true;
  }

  return false;
};
