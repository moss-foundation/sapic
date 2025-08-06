import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { TreeCollectionNode, TreeCollectionRootNode } from "../types";

export const isSourceTreeRootNode = (source: ElementDragPayload): boolean => {
  return source.data.type === "TreeRootNode";
};

//check if all folders are expanded
export const checkIfAllFoldersAreExpanded = (tree: TreeCollectionRootNode): boolean => {
  const checkIfAllNodesAreExpanded = (node: TreeCollectionNode): boolean => {
    if (!node || node.kind === "Item") return true;

    // For folder nodes, check if this node is expanded AND all children are expanded
    if (!node.expanded) return false;

    return node.childNodes.every(checkIfAllNodesAreExpanded);
  };

  return [tree.requests, tree.endpoints, tree.components, tree.schemas].every(checkIfAllNodesAreExpanded);
};

export const checkIfAllFoldersAreCollapsed = (tree: TreeCollectionRootNode): boolean => {
  const checkIfAllNodesAreCollapsed = (node: TreeCollectionNode): boolean => {
    if (!node || node.kind === "Item") return true;

    // For folder nodes, check if this node is collapsed AND all children are collapsed
    if (node.expanded) return false;

    return node.childNodes.every(checkIfAllNodesAreCollapsed);
  };
  return [tree.requests, tree.endpoints, tree.components, tree.schemas].every(checkIfAllNodesAreCollapsed);
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

export const getRestrictedNames = (node: TreeCollectionRootNode, isAddingFolder: boolean) => {
  if (isAddingFolder) {
    return node.requests.childNodes.filter((childNode) => childNode.kind === "Dir").map((childNode) => childNode.name);
  }

  return node.requests.childNodes.filter((childNode) => childNode.kind === "Item").map((childNode) => childNode.name);
};
