import { ResourceNode } from "../ProjectTree/ResourcesTree/types";
import { ProjectTreeRoot } from "../ProjectTree/types";

export const findNodeByIdInTree = (tree: ProjectTreeRoot, id: string): ResourceNode | undefined => {
  for (const child of tree.childNodes) {
    const found = findNodeById(child, id);
    if (found) return found;
  }

  return undefined;
};

export const findNodeById = (topNode: ResourceNode, id: string): ResourceNode | undefined => {
  if (topNode.id === id) return topNode;

  if (topNode.childNodes && topNode.childNodes.length > 0) {
    for (const child of topNode.childNodes) {
      const found = findNodeById(child, id);
      if (found) return found;
    }
  }

  return undefined;
};

export const findNodesSequence = (tree: ProjectTreeRoot, node: ResourceNode) => {
  for (const child of tree.childNodes) {
    const found = findSequence(child, node.path.segments);
    if (found) return found;
  }

  return null;
};

const findSequence = (topNode: ResourceNode, fullPath: string[]) => {
  const nodes: ResourceNode[] = [];

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

export const closeAllNodesInTree = (tree: ResourceNode) => {
  const collapsedTree = { ...tree };
  return collapseAllNodes(collapsedTree);
};

export const collapseAllNodes = <T extends ResourceNode>(node: T): T => {
  return {
    ...node,
    expanded: node.kind === "Dir" ? false : node.expanded,
    childNodes: node.childNodes.map((child) => collapseAllNodes(child)),
  };
};

export const updateTreeNode = (node: ResourceNode, updatedNode: ResourceNode): ResourceNode => {
  if (node.id === updatedNode.id) return updatedNode;

  return {
    ...node,
    childNodes: node.childNodes.map((child) => updateTreeNode(child, updatedNode)),
  };
};
