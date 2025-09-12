import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { TreeCollectionNode, TreeCollectionRootNode } from "../types";

export const isSourceTreeRootNode = (source: ElementDragPayload): boolean => {
  return source.data.type === "TreeRootNode";
};

export const checkIfAllFoldersAreExpanded = (tree: TreeCollectionRootNode): boolean => {
  return true;
  const checkIfAllNodesAreExpanded = (node: TreeCollectionNode): boolean => {
    if (!node || node.kind === "Item") return true;

    if (!node.expanded) return false;

    return node.childNodes.every(checkIfAllNodesAreExpanded);
  };

  return [tree.childNodes].every(checkIfAllNodesAreExpanded);
};

export const checkIfAllFoldersAreCollapsed = (tree: TreeCollectionRootNode): boolean => {
  return false;
  const checkIfAllNodesAreCollapsed = (node: TreeCollectionNode): boolean => {
    if (!node || node.kind === "Item") return true;

    if (node.expanded) return false;

    return node.childNodes.every(checkIfAllNodesAreCollapsed);
  };
  return [tree.childNodes].every(checkIfAllNodesAreCollapsed);
};

export const getTreeRootNodeSourceData = (source: ElementDragPayload) => {
  return source.data as {
    type: "TreeRootNode";
    data: {
      collectionId: string;
      node: TreeCollectionRootNode;
    };
  };
};

export const getTreeRootNodeTargetData = (location: DragLocationHistory) => {
  const instruction = extractInstruction(location.current?.dropTargets[0].data);

  return {
    type: "TreeRootNode",
    data: {
      ...location.current?.dropTargets[0].data,
      instruction,
    },
  } as {
    type: "TreeRootNode";
    data: {
      instruction: Instruction;
      collectionId: string;
      node: TreeCollectionRootNode;
    };
  };
};

export const calculateShouldRenderRootChildNodes = (
  node: TreeCollectionRootNode,
  isDragging: boolean,
  isAddingRootNodeFile: boolean,
  isRenamingRootNode: boolean
) => {
  if (!node.expanded) {
    return false;
  }

  if (isDragging) {
    return false;
  }

  if (isAddingRootNodeFile || isRenamingRootNode) {
    return true;
  }

  return true;
};

export const getChildrenNames = (node: TreeCollectionNode | TreeCollectionRootNode) => {
  return node.childNodes.map((childNode) => childNode.name);
};
