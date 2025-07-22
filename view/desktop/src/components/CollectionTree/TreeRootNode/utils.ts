import { TreeCollectionRootNode } from "../types";

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
