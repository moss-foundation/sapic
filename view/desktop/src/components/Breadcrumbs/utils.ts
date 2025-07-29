import { TreeCollectionNode, TreeCollectionRootNode } from "../CollectionTree/types";

export const findNodeByIdInTree = (tree: TreeCollectionRootNode, id: string): TreeCollectionNode | undefined => {
  const requestRes = findNodeById(tree.requests, id);
  if (requestRes) return requestRes;

  const schemaRes = findNodeById(tree.schemas, id);
  if (schemaRes) return schemaRes;

  const componentRes = findNodeById(tree.components, id);
  if (componentRes) return componentRes;

  const endpointRes = findNodeById(tree.endpoints, id);
  if (endpointRes) return endpointRes;

  return undefined;
};

export const findNodeById = (topNode: TreeCollectionNode, id: string): TreeCollectionNode | undefined => {
  if (topNode.id === id) return topNode;

  if (topNode.childNodes && topNode.childNodes.length > 0) {
    for (const child of topNode.childNodes) {
      const found = findNodeById(child, id);
      if (found) return found;
    }
  }

  return undefined;
};

export const findNodesSequence = (tree: TreeCollectionRootNode, node: TreeCollectionNode) => {
  switch (node.class) {
    case "Endpoint":
      return findSequence(tree.endpoints, node.path.segments);
    case "Schema":
      return findSequence(tree.schemas, node.path.segments);
    case "Component":
      return findSequence(tree.components, node.path.segments);
    case "Request":
      return findSequence(tree.requests, node.path.segments);

    default:
      return null;
  }
};

const findSequence = (topNode: TreeCollectionNode, fullPath: string[]) => {
  const nodes: TreeCollectionNode[] = [];

  if (validSequence(topNode.path.segments, fullPath)) {
    nodes.push(topNode);
  }

  if (topNode.childNodes) {
    for (const child of topNode.childNodes) {
      const result = findSequence(child, fullPath);
      if (result) {
        nodes.push(...result);
      }
    }
  }

  return nodes;
};

const validSequence = (currentPath: string[], fullPath: string[]) => {
  return currentPath.every((item, index) => item === fullPath[index]);
};

export const closeAllNodesInTree = (tree: TreeCollectionNode) => {
  const collapsedTree = { ...tree };
  return collapseAllNodes(collapsedTree);
};

export const collapseAllNodes = <T extends TreeCollectionNode>(node: T): T => {
  return {
    ...node,
    expanded: node.kind === "Dir" ? false : node.expanded,
    childNodes: node.childNodes.map((child) => collapseAllNodes(child)),
  };
};

export const updateTreeNode = (node: TreeCollectionNode, updatedNode: TreeCollectionNode): TreeCollectionNode => {
  if (node.id === updatedNode.id) return updatedNode;

  return {
    ...node,
    childNodes: node.childNodes.map((child) => updateTreeNode(child, updatedNode)),
  };
};
