import { TreeCollectionNode, TreeCollectionRootNode } from "../types";

export const updateNodeInTree = (
  tree: TreeCollectionRootNode,
  updatedNode: TreeCollectionNode
): TreeCollectionRootNode => {
  tree.Requests = updateNodeInArray(tree.Requests, updatedNode);
  tree.Endpoints = updateNodeInArray(tree.Endpoints, updatedNode);
  tree.Components = updateNodeInArray(tree.Components, updatedNode);
  tree.Schemas = updateNodeInArray(tree.Schemas, updatedNode);

  return tree;
};

const updateNodeInArray = (array: TreeCollectionNode[], updatedNode: TreeCollectionNode): TreeCollectionNode[] => {
  return array.map((node) => {
    if (node.id === updatedNode.id) return updatedNode;

    if (node.childNodes.length > 0) {
      node.childNodes = updateNodeInArray(node.childNodes, updatedNode);
    }

    return node;
  });
};
