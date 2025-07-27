import { TreeCollectionNode } from "../types";

export const hasDescendant = (tree: TreeCollectionNode, node: TreeCollectionNode): boolean => {
  if (!tree.childNodes) return false;
  return tree.childNodes.some((child) => child.id === node.id || hasDescendant(child, node));
};

export const hasDirectDescendant = (tree: TreeCollectionNode, node: TreeCollectionNode): boolean => {
  if (!tree.childNodes) return false;
  return tree.childNodes.some((child) => child.id === node.id);
};

export const hasDirectSimilarDescendant = (tree: TreeCollectionNode, node: TreeCollectionNode): boolean => {
  if (!tree.childNodes) return false;
  return tree.childNodes.some((child) => child.id === node.id);
};

const doesStringIncludePartialString = (str: string, partialStr: string) => {
  return str.toLowerCase().includes(partialStr.toLowerCase());
};

export const hasDescendantWithSearchInput = (tree: TreeCollectionNode, input: string): boolean => {
  if (!tree.childNodes) return false;

  const treeId = String(tree.id);

  if (doesStringIncludePartialString(treeId, input)) return true;

  return tree.childNodes.some(
    (child) => doesStringIncludePartialString(treeId, input) || hasDescendantWithSearchInput(child, input)
  );
};

export const countNumberOfAllNestedChildNodes = (node: TreeCollectionNode): number => {
  if (!node.childNodes) return 0;
  return node.childNodes.reduce((acc, child) => acc + 1 + countNumberOfAllNestedChildNodes(child), 0);
};

export const sortByOrder = <T extends { order?: number }>(entries: T[]) => {
  return [...entries].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
};
