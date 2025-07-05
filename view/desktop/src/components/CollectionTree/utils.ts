import { extractInstruction, type Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import {
  DragLocationHistory,
  DropTargetRecord,
  ElementDragPayload,
} from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DropNodeElement, DropNodeElementWithInstruction, TreeCollectionNode } from "./types";

export const updateTreeNode = (node: TreeCollectionNode, updatedNode: TreeCollectionNode): TreeCollectionNode => {
  if (node.id === updatedNode.id) return updateNodeOrder(updatedNode);

  return {
    ...node,
    childNodes: node.childNodes.map((child) => updateTreeNode(child, updatedNode)),
  };
};

export const findNodeById = (tree: TreeCollectionNode, id: string): TreeCollectionNode | undefined => {
  if (tree.id === id) return tree;

  if (tree.childNodes && tree.childNodes.length > 0) {
    for (const child of tree.childNodes) {
      const found = findNodeById(child, id);
      if (found) return found;
    }
  }

  return undefined;
};

export const findNodeByUniqueId = (tree: TreeCollectionNode, id: string): TreeCollectionNode | undefined => {
  if (tree.id === id) return tree;

  if (tree.childNodes && tree.childNodes.length > 0) {
    for (const child of tree.childNodes) {
      const found = findNodeByUniqueId(child, id);
      if (found) return found;
    }
  }

  return undefined;
};

export const findParentNodeByChildUniqueId = (tree: TreeCollectionNode, id: string): TreeCollectionNode | undefined => {
  if (tree.childNodes.some((child) => child.id === id)) {
    return tree;
  }

  for (const child of tree.childNodes) {
    const parent = findParentNodeByChildUniqueId(child, id);

    if (parent !== undefined) {
      return parent;
    }
  }

  return undefined;
};

export const hasDescendant = (tree: TreeCollectionNode, node: TreeCollectionNode): boolean => {
  if (!tree.childNodes) return false;
  return tree.childNodes.some((child) => child.id === node.id || hasDescendant(child, node));
};

export const hasDirectDescendant = (tree: TreeCollectionNode, node: TreeCollectionNode): boolean => {
  if (!tree.childNodes) return false;
  return tree.childNodes.some((child) => child.id === node.id && child.id === node.id);
};

export const hasDirectSimilarDescendant = (tree: TreeCollectionNode, node: TreeCollectionNode): boolean => {
  if (!tree.childNodes) return false;
  return tree.childNodes.some((child) => child.id === node.id || child.id === node.id);
};

const doesStringIncludePartialString = (str: string, partialStr: string) => {
  return str.toLowerCase().includes(partialStr.toLowerCase());
};

export const hasDescendantWithSearchInput = (tree: TreeCollectionNode, input: string): boolean => {
  if (!tree.childNodes) return false;

  const treeId = String(tree.id);

  if (doesStringIncludePartialString(treeId, input)) return true;

  return tree.childNodes.some(
    (child) => doesStringIncludePartialString(treeId, input) || hasDescendantWithSearchInput(child, input)
  );
};

export const addNodeToFolder = (
  tree: TreeCollectionNode,
  targetUniqueId: string,
  nodeToAdd: TreeCollectionNode
): TreeCollectionNode => {
  if (tree.id === targetUniqueId) {
    return updateNodeOrder({
      ...tree,
      childNodes: [...tree.childNodes, nodeToAdd],
    });
  }

  return {
    ...tree,
    childNodes: tree.childNodes.map((child) => addNodeToFolder(child, targetUniqueId, nodeToAdd)),
  };
};

export const addNodeChildrenWithInstruction = (
  tree: TreeCollectionNode,
  targetUniqueId: string,
  childNodes: TreeCollectionNode[],
  instruction: Instruction
): TreeCollectionNode => {
  if (tree.id === targetUniqueId) {
    return updateNodeOrder({
      ...tree,
      childNodes,
    });
  }

  return {
    ...tree,
    childNodes: tree.childNodes.map((child) =>
      addNodeChildrenWithInstruction(child, targetUniqueId, childNodes, instruction)
    ),
  };
};

export const getActualDropSourceTarget = (source: ElementDragPayload): DropNodeElement => {
  return source.data.data as DropNodeElement;
};

export const getActualDropTarget = (location: DragLocationHistory): DropNodeElement => {
  return (location.current.dropTargets[0].data.data as DropNodeElement).node.isFolder
    ? (location.current.dropTargets[0].data.data as DropNodeElement)
    : (location.current.dropTargets[1].data.data as DropNodeElement);
};

export const getActualDropTargetWithInstruction = (
  location: DragLocationHistory,
  self: DropTargetRecord
): {
  dropTarget: DropNodeElementWithInstruction;
  instruction: Instruction | null;
} => {
  const instruction = extractInstruction(self.data);

  return {
    dropTarget: location.current.dropTargets[0].data.data as unknown as DropNodeElementWithInstruction,
    instruction,
  };
};

export const addNodeToTreeWithInstruction = (
  tree: TreeCollectionNode,
  targetNode: TreeCollectionNode,
  sourceNode: TreeCollectionNode,
  instruction: Instruction | undefined
): TreeCollectionNode => {
  const treeWithoutSource = removeNodeFromTree(tree, sourceNode.id);

  if (!instruction) {
    if (targetNode.kind === "Dir") {
      return addNodeToFolder(treeWithoutSource, targetNode.id, sourceNode);
    }

    return tree;
  }

  if (instruction.operation === "combine" && targetNode.kind === "Dir") {
    return addNodeToFolder(treeWithoutSource, targetNode.id, sourceNode);
  }

  const targetParentNode = findParentNodeByChildUniqueId(treeWithoutSource, targetNode.id);
  if (!targetParentNode) return treeWithoutSource;

  const indexOfTargetNode = targetParentNode.childNodes.findIndex((child) => child.id === targetNode.id);
  if (indexOfTargetNode === -1) return treeWithoutSource;

  if (instruction.operation === "reorder-before") {
    return addNodeChildrenWithInstruction(
      treeWithoutSource,
      targetParentNode.id,
      [
        ...targetParentNode.childNodes.slice(0, indexOfTargetNode),
        sourceNode,
        ...targetParentNode.childNodes.slice(indexOfTargetNode),
      ],
      instruction
    );
  }

  if (instruction.operation === "reorder-after") {
    return addNodeChildrenWithInstruction(
      treeWithoutSource,
      targetParentNode.id,
      [
        ...targetParentNode.childNodes.slice(0, indexOfTargetNode + 1),
        sourceNode,
        ...targetParentNode.childNodes.slice(indexOfTargetNode + 1),
      ],
      instruction
    );
  }

  return tree;
};

export const canDropNode = (sourceTarget: DropNodeElement, dropTarget: DropNodeElement, node: TreeCollectionNode) => {
  if (sourceTarget.node.isFolder === false) {
    // if (hasDirectSimilarDescendant(node, sourceTarget.node)) {
    //   return false;
    // }
  }

  if (sourceTarget.node.isFolder) {
    // if (hasDirectSimilarDescendant(node, sourceTarget.node)) {
    //   return false;
    // }

    if (hasDirectDescendant(dropTarget.node, node)) {
      return false;
    }

    if (hasDescendant(sourceTarget.node, node)) {
      return false;
    }

    if (sourceTarget?.node.id === node.id) {
      return false;
    }
  }

  return true;
};

export const checkIfAllFoldersAreExpanded = <T extends TreeCollectionNode>(nodes: T[]): boolean => {
  if (!nodes || nodes.length === 0) return true;

  for (const node of nodes) {
    if (node.kind === "Dir" && !node.expanded) {
      return false;
    }
    if (!checkIfAllFoldersAreExpanded(node.childNodes)) {
      return false;
    }
  }

  return true;
};

export const checkIfAllFoldersAreCollapsed = <T extends TreeCollectionNode>(nodes: T[]): boolean => {
  if (!nodes || nodes.length === 0) return true;

  for (const node of nodes) {
    if (node.kind === "Dir" && node.expanded) {
      return false;
    }
    if (!checkIfAllFoldersAreCollapsed(node.childNodes)) {
      return false;
    }
  }

  return true;
};

export const checkIfTreeIsCollapsed = <T extends TreeCollectionNode>(node: T): boolean => {
  if (node.expanded) return false;

  if (node.childNodes && node.childNodes.length > 0) {
    for (const child of node.childNodes) {
      if (!checkIfTreeIsCollapsed(child)) {
        return false;
      }
    }
  }
  return true;
};

export const checkIfTreeIsExpanded = <T extends TreeCollectionNode>(node: T): boolean => {
  if (!node.expanded) return false;

  if (node.childNodes && node.childNodes.length > 0) {
    for (const child of node.childNodes) {
      if (!checkIfTreeIsExpanded(child)) {
        return false;
      }
    }
  }

  return true;
};

export const updateNodeOrder = (node: TreeCollectionNode): TreeCollectionNode => {
  return {
    ...node,
    childNodes: node.childNodes.map((child, index) => ({
      ...child,
      order: index + 1,
    })),
  };
};

export const updateNodesOrder = (nodes: TreeCollectionNode[]): TreeCollectionNode[] => {
  return nodes.map((node, index) => ({
    ...node,
    order: index + 1,
  }));
};

export const sortNodesByOrder = (nodes: TreeCollectionNode[]): TreeCollectionNode[] => {
  //FIXME nodes without order shouldnt exist, it's a temporary check
  return [...nodes].sort((a, b) => {
    const validOrderA = typeof a.order === "number";
    const validOrderB = typeof b.order === "number";

    if (validOrderA && validOrderB) {
      return a.order! - b.order!;
    }

    if (validOrderA) return -1;
    if (validOrderB) return 1;

    return a.name.localeCompare(b.name, undefined, { sensitivity: "base" });
  });
};
