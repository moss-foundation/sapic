import { ProjectTreeNode, ProjectTreeRootNode } from "../ProjectTree/types";

export const findNodeByIdInTree = (tree: ProjectTreeRootNode, id: string): ProjectTreeNode | undefined => {
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

export const findNodeById = (topNode: ProjectTreeNode, id: string): ProjectTreeNode | undefined => {
  if (topNode.id === id) return topNode;

  if (topNode.childNodes && topNode.childNodes.length > 0) {
    for (const child of topNode.childNodes) {
      const found = findNodeById(child, id);
      if (found) return found;
    }
  }

  return undefined;
};

export const findNodesSequence = (tree: ProjectTreeRootNode, node: ProjectTreeNode) => {
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

const findSequence = (topNode: ProjectTreeNode, fullPath: string[]) => {
  const nodes: ProjectTreeNode[] = [];

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

export const closeAllNodesInTree = (tree: ProjectTreeNode) => {
  const collapsedTree = { ...tree };
  return collapseAllNodes(collapsedTree);
};

export const collapseAllNodes = <T extends ProjectTreeNode>(node: T): T => {
  return {
    ...node,
    expanded: node.kind === "Dir" ? false : node.expanded,
    childNodes: node.childNodes.map((child) => collapseAllNodes(child)),
  };
};

export const updateTreeNode = (node: ProjectTreeNode, updatedNode: ProjectTreeNode): ProjectTreeNode => {
  if (node.id === updatedNode.id) return updatedNode;

  return {
    ...node,
    childNodes: node.childNodes.map((child) => updateTreeNode(child, updatedNode)),
  };
};
