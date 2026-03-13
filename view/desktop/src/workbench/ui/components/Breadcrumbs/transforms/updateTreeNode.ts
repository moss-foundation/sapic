import { ResourceNode } from "../../ProjectTree/ResourcesTree/types";

export const updateTreeNode = (node: ResourceNode, updatedNode: ResourceNode): ResourceNode => {
  if (node.id === updatedNode.id) return updatedNode;

  return {
    ...node,
    childNodes: node.childNodes.map((child) => updateTreeNode(child, updatedNode)),
  };
};
