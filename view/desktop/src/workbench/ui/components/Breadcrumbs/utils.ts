import { ProjectTreeNode, ProjectTreeRootNode } from "../ProjectTree/types";

export const findNodeByIdInTree = (tree: ProjectTreeRootNode, id: string): ProjectTreeNode | undefined => {
  for (const child of tree.childNodes) {
    const found = findNodeById(child, id);
    if (found) return found;
  }

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
  for (const child of tree.childNodes) {
    const found = findSequence(child, node.path.segments);
    if (found) return found;
  }

  return null;
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
