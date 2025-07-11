import { extractInstruction, type Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import {
  DragLocationHistory,
  DropTargetRecord,
  ElementDragPayload,
} from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragNode, DropNodeElementWithInstruction, SortTypes, TreeCollectionNode } from "./types";

export const updateTreeNode = (node: TreeCollectionNode, updatedNode: TreeCollectionNode): TreeCollectionNode => {
  if (node.id === updatedNode.id) return updateNodeOrder(updatedNode);

  return {
    ...node,
    childNodes: node.childNodes.map((child) => updateTreeNode(child, updatedNode)),
  };
};

export const sortNode = (node: TreeCollectionNode, sortBy: SortTypes = "alphabetically"): TreeCollectionNode => {
  if (sortBy === "none") return node;

  return {
    ...node,
    childNodes: sortNodes(
      node.childNodes.map((child) => sortNode(child, sortBy)),
      sortBy
    ),
  };
};

export const sortNodes = (nodes: TreeCollectionNode[], sortBy: SortTypes = "alphabetically"): TreeCollectionNode[] => {
  if (sortBy === "alphabetically") {
    nodes.sort((a, b) => {
      if (a.kind === "Dir" && b.kind === "Item") return -1;
      if (a.kind === "Item" && b.kind === "Dir") return 1;
      if (a.id < b.id) return -1;
      if (a.id > b.id) return 1;
      return 0;
    });

    return nodes.map((node, index) => {
      return {
        ...node,
        order: index + 1,
      };
    });
  }

  if (sortBy === "order") {
    nodes.sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
    return nodes;
  }

  return nodes;
};

export const prepareCollectionForTree = (
  collection: TreeCollectionNode,
  sortBy: SortTypes = "none"
): TreeCollectionNode => {
  return sortNode(
    {
      ...collection,
      childNodes: collection.childNodes.map((child) => prepareCollectionForTree(child, sortBy)),
    },
    sortBy
  );
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

export const removeNodeFromTree = (tree: TreeCollectionNode, id: string): TreeCollectionNode => {
  if (tree.childNodes.some((child) => child.id === id)) {
    return {
      ...tree,
      childNodes: tree.childNodes.filter((child) => child.id !== id),
    };
  }

  return {
    ...tree,
    childNodes: tree.childNodes.map((child) => removeNodeFromTree(child, id)),
  };
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

export const getActualDropSourceTarget = (source: ElementDragPayload): DragNode => {
  return source.data.data as DragNode;
};

export const getActualDropTarget = (location: DragLocationHistory): DragNode => {
  return (location.current.dropTargets[0].data.data as DragNode).node.kind === "Dir"
    ? (location.current.dropTargets[0].data.data as DragNode)
    : (location.current.dropTargets[1].data.data as DragNode);
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

export const canDropNode = (sourceTarget: DragNode, dropTarget: DragNode, node: TreeCollectionNode) => {
  if (sourceTarget.node.kind !== "Dir") {
    // if (hasDirectSimilarDescendant(node, sourceTarget.node)) {
    //   return false;
    // }
  }

  if (sourceTarget.node.kind === "Dir") {
    // if (hasDirectSimilarDescendant(node, sourceTarget.node)) {
    //   return false;
    // }

    if (hasDirectDescendant(dropTarget.node, node)) {
      return false;
    }

    if (hasDescendant(sourceTarget.node, node)) {
      return false;
    }

    if (sourceTarget.node.id === node.id) {
      return false;
    }
  }

  return true;
};

export const expandAllNodes = <T extends TreeCollectionNode>(node: T): T => {
  return {
    ...node,
    expanded: node.kind === "Dir" ? true : node.expanded,
    childNodes: node.childNodes.map((child) => expandAllNodes(child)) as T["childNodes"],
  };
};

export const collapseAllNodes = <T extends TreeCollectionNode>(node: T): T => {
  return {
    ...node,
    expanded: node.kind === "Dir" ? false : node.expanded,
    childNodes: node.childNodes.map((child) => collapseAllNodes(child)),
  };
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
