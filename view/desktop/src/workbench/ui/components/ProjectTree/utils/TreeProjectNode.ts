import { ResourceNode } from "../ResourcesTree/types";

export const hasDescendant = (sourceNode: ResourceNode, locationNode: ResourceNode): boolean => {
  if (sourceNode.id === locationNode.id) return true;

  return sourceNode.childNodes.some((child) => {
    if (child.id === locationNode.id) return true;
    return hasDescendant(child, locationNode);
  });
};

export const hasDirectDescendant = (parentNode: ResourceNode, dropNode: ResourceNode): boolean => {
  if (!parentNode.childNodes) return false;
  return parentNode.childNodes.some((child) => child.id === dropNode.id);
};

const doesStringIncludePartialString = (str: string, partialStr: string) => {
  return str.toLowerCase().includes(partialStr.toLowerCase());
};

export const hasDescendantWithSearchInput = (parentNode: ResourceNode, input: string): boolean => {
  if (!parentNode.childNodes) return false;

  const projectId = String(parentNode.id);

  if (doesStringIncludePartialString(projectId, input)) return true;

  return parentNode.childNodes.some(
    (child) => doesStringIncludePartialString(projectId, input) || hasDescendantWithSearchInput(child, input)
  );
};

export const countNumberOfAllNestedChildNodes = (node: ResourceNode): number => {
  if (!node.childNodes) return 0;
  return node.childNodes.reduce((acc, child) => {
    const childCount = child.kind === "Item" ? 1 : 0;
    return acc + childCount + countNumberOfAllNestedChildNodes(child);
  }, 0);
};
