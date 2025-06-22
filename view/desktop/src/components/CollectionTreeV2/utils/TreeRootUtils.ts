import { TreeCollectionNode, TreeCollectionRootNode } from "../types";

export const updateNodeInTree = (
  tree: TreeCollectionRootNode,
  updatedNode: TreeCollectionNode
): TreeCollectionRootNode => {
  switch (updatedNode.class) {
    case "Request":
      tree.requests = updateNode(tree.requests, updatedNode);
      break;
    case "Endpoint":
      tree.endpoints = updateNode(tree.endpoints, updatedNode);
      break;
    case "Component":
      tree.components = updateNode(tree.components, updatedNode);
      break;
    case "Schema":
      tree.schemas = updateNode(tree.schemas, updatedNode);
      break;
  }

  return tree;
};

const updateNode = (node: TreeCollectionNode, updatedNode: TreeCollectionNode): TreeCollectionNode => {
  if (node.id === updatedNode.id) return updatedNode;

  if (node.childNodes.length > 0) {
    node.childNodes = updateNodeInArray(node.childNodes, updatedNode);
  }

  return node;
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

//check if all folders are expanded
export const checkIfAllFoldersAreExpanded = (tree: TreeCollectionRootNode): boolean => {
  return [tree.requests, tree.endpoints, tree.components, tree.schemas].every((node) => node.expanded);
};

export const checkIfAllFoldersAreCollapsed = (tree: TreeCollectionRootNode): boolean => {
  return [tree.requests, tree.endpoints, tree.components, tree.schemas].every((node) => !node.expanded);
};

//expand all nodes
export const expandAllNodes = (tree: TreeCollectionRootNode): TreeCollectionRootNode => {
  return {
    ...tree,
    requests: expandCollectionNodes(tree.requests),
    endpoints: expandCollectionNodes(tree.endpoints),
    components: expandCollectionNodes(tree.components),
    schemas: expandCollectionNodes(tree.schemas),
  };
};

const expandCollectionNodes = (node: TreeCollectionNode): TreeCollectionNode => {
  return {
    ...node,
    expanded: true,
    childNodes: node.childNodes.map(expandCollectionNodes),
  };
};

//collapse all nodes
export const collapseAllNodes = (tree: TreeCollectionRootNode): TreeCollectionRootNode => {
  return {
    ...tree,
    requests: collapseCollectionNodes(tree.requests),
    endpoints: collapseCollectionNodes(tree.endpoints),
    components: collapseCollectionNodes(tree.components),
    schemas: collapseCollectionNodes(tree.schemas),
  };
};

const collapseCollectionNodes = (node: TreeCollectionNode): TreeCollectionNode => {
  return {
    ...node,
    expanded: false,
    childNodes: node.childNodes.map(collapseCollectionNodes),
  };
};
