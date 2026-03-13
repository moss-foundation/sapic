import { ResourceNode } from "../../types";

export const hasDescendant = (sourceNode: ResourceNode, locationNode: ResourceNode): boolean => {
  if (sourceNode.id === locationNode.id) return true;

  return sourceNode.childNodes.some((child) => {
    if (child.id === locationNode.id) return true;
    return hasDescendant(child, locationNode);
  });
};
