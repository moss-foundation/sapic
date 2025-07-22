import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { DragNode, DropNode, TreeCollectionNode } from "./types";

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

export const getActualDropSourceTarget = (source: ElementDragPayload): DragNode => {
  return source.data.data as DragNode;
};

export const getActualDropTarget = (location: DragLocationHistory): DragNode => {
  return (location.current.dropTargets[0].data.data as DragNode).node.kind === "Dir"
    ? (location.current.dropTargets[0].data.data as DragNode)
    : (location.current.dropTargets[1].data.data as DragNode);
};

export const canDropNode = (sourceTarget: DragNode, dropTarget: DropNode) => {
  if (sourceTarget.node.class !== dropTarget.node.class) {
    return false;
  }

  if (sourceTarget.node.kind === "Dir") {
    if (sourceTarget.node.id === dropTarget.node.id) {
      return false;
    }

    if (hasDirectDescendant(dropTarget.node, sourceTarget.node)) {
      return false;
    }

    if (hasDescendant(dropTarget.node, sourceTarget.node)) {
      return false;
    }
  }

  return true;
};
