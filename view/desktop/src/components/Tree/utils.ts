import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/tree-item";
import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/tree-item";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DropNodeElement, DropNodeElementWithInstruction, NodeProps, SortTypes, TreeNodeProps } from "./types";

export const updateTreeNode = (node: TreeNodeProps, updatedNode: TreeNodeProps): TreeNodeProps => {
  if (node.uniqueId === updatedNode.uniqueId) return updateNodeOrder(updatedNode);

  return {
    ...node,
    childNodes: node.childNodes.map((child) => updateTreeNode(child, updatedNode)),
  };
};

export const sortNode = (node: TreeNodeProps, sortBy: SortTypes = "alphabetically"): TreeNodeProps => {
  if (sortBy === "none") return node;

  return {
    ...node,
    childNodes: sortNodes(
      node.childNodes.map((child) => sortNode(child, sortBy)),
      sortBy
    ),
  };
};

export const sortNodes = (nodes: TreeNodeProps[], sortBy: SortTypes = "alphabetically"): TreeNodeProps[] => {
  if (sortBy === "alphabetically") {
    nodes.sort((a, b) => {
      if (a.isFolder && !b.isFolder) return -1;
      if (!a.isFolder && b.isFolder) return 1;
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
    nodes.sort((a, b) => a.order - b.order);
    return nodes;
  }

  return nodes;
};

export const prepareCollectionForTree = (
  collection: NodeProps,
  sortBy: SortTypes = "none",
  isFirstCollection: boolean = true
): TreeNodeProps => {
  const id = "TreeNodeUniqueId-" + Math.random().toString(36).substring(2, 15);

  return sortNode(
    {
      ...collection,
      uniqueId: id,
      isRoot: isFirstCollection,
      childNodes: collection.childNodes.map((child) => prepareCollectionForTree(child, sortBy, false)),
    },
    sortBy
  );
};

export const removeUniqueIdFromTree = (tree: TreeNodeProps): NodeProps => {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { uniqueId, ...treeWithoutUniqueId } = tree;

  return {
    ...treeWithoutUniqueId,
    childNodes: tree.childNodes.map((child) => removeUniqueIdFromTree(child)),
  };
};

export const findNodeById = (tree: NodeProps, id: string): NodeProps | undefined => {
  if (tree.id === id) return tree;

  if (tree.childNodes && tree.childNodes.length > 0) {
    for (const child of tree.childNodes) {
      const found = findNodeById(child, id);
      if (found) return found;
    }
  }

  return undefined;
};

export const findNodeByUniqueId = (tree: TreeNodeProps, uniqueId: string): TreeNodeProps | undefined => {
  if (tree.uniqueId === uniqueId) return tree;

  if (tree.childNodes && tree.childNodes.length > 0) {
    for (const child of tree.childNodes) {
      const found = findNodeByUniqueId(child, uniqueId);
      if (found) return found;
    }
  }

  return undefined;
};

export const findParentNodeByChildUniqueId = (tree: TreeNodeProps, uniqueId: string): TreeNodeProps | undefined => {
  if (tree.childNodes.some((child) => child.uniqueId === uniqueId)) {
    return tree;
  }

  for (const child of tree.childNodes) {
    const parent = findParentNodeByChildUniqueId(child, uniqueId);

    if (parent !== undefined) {
      return parent;
    }
  }

  return undefined;
};

export const hasDescendant = (tree: TreeNodeProps, node: TreeNodeProps): boolean => {
  if (!tree.childNodes) return false;
  return tree.childNodes.some((child) => child.uniqueId === node.uniqueId || hasDescendant(child, node));
};

export const hasDirectDescendant = (tree: TreeNodeProps, node: TreeNodeProps): boolean => {
  if (!tree.childNodes) return false;
  return tree.childNodes.some((child) => child.uniqueId === node.uniqueId && child.id === node.id);
};

export const hasDirectSimilarDescendant = (tree: TreeNodeProps, node: TreeNodeProps): boolean => {
  if (!tree.childNodes) return false;
  return tree.childNodes.some((child) => child.uniqueId === node.uniqueId || child.id === node.id);
};

const doesStringIncludePartialString = (str: string, partialStr: string) => {
  return str.toLowerCase().includes(partialStr.toLowerCase());
};

export const hasDescendantWithSearchInput = (tree: TreeNodeProps, input: string): boolean => {
  if (!tree.childNodes) return false;

  const treeId = String(tree.id);

  if (doesStringIncludePartialString(treeId, input)) return true;

  return tree.childNodes.some(
    (child) => doesStringIncludePartialString(treeId, input) || hasDescendantWithSearchInput(child, input)
  );
};

export const removeNodeFromTree = (tree: TreeNodeProps, uniqueId: string): TreeNodeProps => {
  if (tree.childNodes.some((child) => child.uniqueId === uniqueId)) {
    return {
      ...tree,
      childNodes: tree.childNodes.filter((child) => child.uniqueId !== uniqueId),
    };
  }

  return {
    ...tree,
    childNodes: tree.childNodes.map((child) => removeNodeFromTree(child, uniqueId)),
  };
};

export const addNodeToFolder = (
  tree: TreeNodeProps,
  targetUniqueId: string,
  nodeToAdd: TreeNodeProps
): TreeNodeProps => {
  if (tree.uniqueId === targetUniqueId) {
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
  tree: TreeNodeProps,
  targetUniqueId: string,
  childNodes: TreeNodeProps[],
  instruction: Instruction
): TreeNodeProps => {
  if (tree.uniqueId === targetUniqueId) {
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
  location: DragLocationHistory
): {
  dropTarget: DropNodeElementWithInstruction;
  instruction: Instruction | null;
} => {
  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    dropTarget: location.current.dropTargets[0].data.data as unknown as DropNodeElementWithInstruction,
    instruction,
  };
};

export const canDropNode = (sourceTarget: DropNodeElement, dropTarget: DropNodeElement, node: TreeNodeProps) => {
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

    if (sourceTarget?.node.uniqueId === node.uniqueId) {
      return false;
    }
  }

  return true;
};

export const expandAllNodes = <T extends NodeProps>(node: T): T => {
  return {
    ...node,
    isExpanded: node.isFolder ? true : node.isExpanded,
    childNodes: node.childNodes.map((child) => expandAllNodes(child)) as T["childNodes"],
  };
};

export const collapseAllNodes = <T extends NodeProps>(node: T): T => {
  return {
    ...node,
    isExpanded: node.isFolder ? false : node.isExpanded,
    childNodes: node.childNodes.map((child) => collapseAllNodes(child)),
  };
};

export const checkIfAllFoldersAreExpanded = <T extends NodeProps>(nodes: T[]): boolean => {
  if (!nodes || nodes.length === 0) return true;

  for (const node of nodes) {
    if (node.isFolder && !node.isExpanded) {
      return false;
    }
    if (!checkIfAllFoldersAreExpanded(node.childNodes)) {
      return false;
    }
  }

  return true;
};

export const checkIfAllFoldersAreCollapsed = <T extends NodeProps>(nodes: T[]): boolean => {
  if (!nodes || nodes.length === 0) return true;

  for (const node of nodes) {
    if (node.isFolder && node.isExpanded) {
      return false;
    }
    if (!checkIfAllFoldersAreCollapsed(node.childNodes)) {
      return false;
    }
  }

  return true;
};

export const checkIfTreeIsCollapsed = <T extends NodeProps>(node: T): boolean => {
  if (node.isExpanded) return false;

  if (node.childNodes && node.childNodes.length > 0) {
    for (const child of node.childNodes) {
      if (!checkIfTreeIsCollapsed(child)) {
        return false;
      }
    }
  }
  return true;
};

export const checkIfTreeIsExpanded = <T extends NodeProps>(node: T): boolean => {
  if (!node.isExpanded) return false;

  if (node.childNodes && node.childNodes.length > 0) {
    for (const child of node.childNodes) {
      if (!checkIfTreeIsExpanded(child)) {
        return false;
      }
    }
  }

  return true;
};

export const updateNodeOrder = (node: TreeNodeProps): TreeNodeProps => {
  return {
    ...node,
    childNodes: node.childNodes.map((child, index) => ({
      ...child,
      order: index + 1,
    })),
  };
};

export const updateNodesOrder = (nodes: TreeNodeProps[]): TreeNodeProps[] => {
  return nodes.map((node, index) => ({
    ...node,
    order: index + 1,
  }));
};
