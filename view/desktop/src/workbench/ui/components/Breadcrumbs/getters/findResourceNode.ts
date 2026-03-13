import { ResourceNode } from "../../ProjectTree/ResourcesTree/types";
import { ProjectTreeRoot } from "../../ProjectTree/types";

export const findNodeByIdInTree = (tree: ProjectTreeRoot, id: string): ResourceNode | undefined => {
  for (const child of tree.resourcesTree.childNodes) {
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
  for (const child of tree.resourcesTree.childNodes) {
    const found = findSequence(child, node.path.segments);
    if (found) return found;
  }

  return null;
};

const findSequence = (topNode: ResourceNode, fullPath: string[]) => {
  const nodes: ResourceNode[] = [];

  if (isValidSequence(topNode.path.segments, fullPath)) {
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

const isValidSequence = (currentPath: string[], fullPath: string[]) => {
  return currentPath.every((item, index) => item === fullPath[index]);
};
