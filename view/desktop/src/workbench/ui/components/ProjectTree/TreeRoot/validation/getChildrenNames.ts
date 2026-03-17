import { ResourceNode } from "../../ResourcesTree/types";

export const getChildrenNames = (node: ResourceNode) => {
  return node.childNodes.map((childNode) => childNode.name);
};
