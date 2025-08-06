import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import {
  DragLocationHistory,
  DropTargetRecord,
  ElementDragPayload,
} from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { EntryInfo } from "@repo/moss-collection";

import { DragNode, DropNode, DropRootNode, TreeCollectionNode } from "../types";
import { hasDescendant, hasDirectSimilarDescendant } from "./TreeNode";

//source
export const getSourceTreeNodeData = (source: ElementDragPayload): DragNode | null => {
  if (source.data.type !== "TreeNode") {
    return null;
  }

  return source.data.data as DragNode;
};

export const isSourceTreeNode = (source: ElementDragPayload): boolean => {
  return source.data.type === "TreeNode";
};

//location
export const getLocationTreeNodeData = (location: DragLocationHistory): DropNode | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== "TreeNode") return null;

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
export const doesLocationHaveTreeNode = (location: DragLocationHistory): boolean => {
  if (location.current.dropTargets.length === 0) return false;
  return location.current.dropTargets[0].data.type === "TreeNode";
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
    // console.log("can't drop: class mismatch");
    return false;
  }

  if (sourceTarget.node.id === dropTarget.node.id) {
    // console.log("can't drop: id mismatch");
    return false;
  }

  if (dropTarget.node.kind === "Dir") {
    if (hasDescendant(dropTarget.node, sourceTarget.node)) {
      // console.log("can't drop: has direct descendant");
      return false;
    }

    if (hasDirectSimilarDescendant(dropTarget.node, sourceTarget.node)) {
      // console.log("can't drop: has direct similar descendant");
      return false;
    }
  }

  if (dropTarget.node.kind === "Item") {
    if (hasDirectSimilarDescendant(dropTarget.parentNode, sourceTarget.node)) {
      // console.log("can't drop: has direct similar descendant");
      return false;
    }
  }

  return true;
};
