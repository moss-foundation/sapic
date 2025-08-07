import { TreeCollectionNode } from "../types";

export const hasDescendant = (parentNode: TreeCollectionNode, dropNode: TreeCollectionNode): boolean => {
  if (parentNode.id === dropNode.id) return true;
  if (parentNode.childNodes.length === 0) return false;

  return parentNode.childNodes.some((child) => {
    if (child.id === dropNode.id) return true;
    return hasDescendant(child, dropNode);
  });
};

export const hasDirectDescendant = (parentNode: TreeCollectionNode, dropNode: TreeCollectionNode): boolean => {
  if (!parentNode.childNodes) return false;
  return parentNode.childNodes.some((child) => child.id === dropNode.id);
};

export const hasDirectSimilarDescendant = (parentNode: TreeCollectionNode, dropNode: TreeCollectionNode): boolean => {
  if (!parentNode.childNodes) return false;
  return parentNode.childNodes.some(
    (child) => child.id === dropNode.id || child.name.toLowerCase() === dropNode.name.toLowerCase()
  );
};

export const hasAnotherDirectDescendantWithSimilarName = (
  parentNode: TreeCollectionNode,
  dropNode: TreeCollectionNode
): boolean => {
  if (!parentNode.childNodes) return false;
  return parentNode.childNodes.some(
    (child) => child.id !== dropNode.id && child.name.toLowerCase() === dropNode.name.toLowerCase()
  );
};

const doesStringIncludePartialString = (str: string, partialStr: string) => {
  return str.toLowerCase().includes(partialStr.toLowerCase());
};

export const hasDescendantWithSearchInput = (parentNode: TreeCollectionNode, input: string): boolean => {
  if (!parentNode.childNodes) return false;

  const collectionId = String(parentNode.id);

  if (doesStringIncludePartialString(collectionId, input)) return true;

  return parentNode.childNodes.some(
    (child) => doesStringIncludePartialString(collectionId, input) || hasDescendantWithSearchInput(child, input)
  );
};

export const countNumberOfAllNestedChildNodes = (node: TreeCollectionNode): number => {
  if (!node.childNodes) return 0;
  return node.childNodes.reduce((acc, child) => acc + 1 + countNumberOfAllNestedChildNodes(child), 0);
};

export const sortByOrder = <T extends { order?: number }>(entries: T[]) => {
  return [...entries].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
};
