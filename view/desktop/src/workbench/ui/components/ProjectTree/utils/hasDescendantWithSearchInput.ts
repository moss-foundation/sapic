import { ResourceNode } from "../ResourcesTree/types";

export const hasDescendantWithSearchInput = (parentNode: ResourceNode, input: string): boolean => {
  if (!parentNode.childNodes) return false;

  const projectId = String(parentNode.id);

  if (doesStringIncludePartialString(projectId, input)) return true;

  return parentNode.childNodes.some(
    (child) => doesStringIncludePartialString(projectId, input) || hasDescendantWithSearchInput(child, input)
  );
};

const doesStringIncludePartialString = (str: string, partialStr: string) => {
  return str.toLowerCase().includes(partialStr.toLowerCase());
};
